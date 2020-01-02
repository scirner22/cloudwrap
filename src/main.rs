extern crate chrono;
#[macro_use]
extern crate clap;
extern crate ctrlc;
extern crate openssl_probe;
#[macro_use]
extern crate prettytable;
extern crate rusoto_core;
extern crate rusoto_secretsmanager;
extern crate rusoto_ssm;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::{
    collections::HashSet, fs::File, hash::Hash, io::prelude::*, path::PathBuf, process::Command,
    sync::atomic, sync::Arc, thread, time::Duration,
};

use clap::App;

mod config;
mod error;
mod output;
mod secretsmanager;
mod ssm;
mod types;

use config::Config;
use error::Error;
use output::{Exportable, /*Postgres,*/ Printable};
use types::Result;

fn spawn_signal_handler() -> () {
    let running_state = Arc::new(atomic::AtomicBool::new(true));
    let shared_state = running_state.clone();

    ctrlc::set_handler(move || {
        shared_state.store(false, atomic::Ordering::SeqCst);
    })
    .unwrap();

    while running_state.load(atomic::Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(5));
    }
}

fn extend<T>(set: &mut HashSet<T>, fields: Vec<T>) -> ()
where
    T: Hash + Eq,
{
    for field in fields {
        set.insert(field);
    }
}

fn output_describe(configs: &Vec<Config>) -> Result<()> {
    let ssm_client = ssm::SsmClient::default();
    let mut ssm = HashSet::new();

    for config in configs {
        let fields = ssm_client.describe_parameters(config)?;
        extend(&mut ssm, fields);
    }

    let ssm = ssm.into_iter().collect::<Vec<_>>();
    ssm.get_table().printstd();

    Ok(())
}

fn output_stdout(configs: &Vec<Config>) -> Result<()> {
    let ssm_client = ssm::SsmClient::default();
    let mut ssm = HashSet::new();

    for config in configs {
        let fields = ssm_client.get_parameters(config)?;
        extend(&mut ssm, fields);
    }

    let mut closure = move |pairs: Vec<(String, String)>| {
        for (k, v) in pairs {
            println!("{}={}", k, v);
        }
    };

    let ssm = ssm.into_iter().collect::<Vec<_>>();
    ssm.export().map(&mut closure);

    Ok(())
}

fn output_file<S>(configs: &Vec<Config>, path: S) -> Result<()>
where
    S: Into<PathBuf>,
{
    let path = path.into();
    let ssm_client = ssm::SsmClient::default();
    let mut ssm = HashSet::new();

    for config in configs {
        let fields = ssm_client.get_parameters(config)?;
        extend(&mut ssm, fields);
    }

    path.parent().map(|p| {
        if !p.exists() {
            panic!(format!("{:?} does not exist", p))
        }
    });

    let mut file = File::create(path).expect("opening file");
    let mut closure = move |pairs: Vec<(String, String)>| {
        for (k, v) in pairs {
            file.write_all(format!("export {}={}\n", k, v).as_bytes())
                .expect("writing to file");
        }
    };

    let ssm = ssm.into_iter().collect::<Vec<_>>();
    ssm.export().map(&mut closure);

    Ok(())
}

fn run_command(command: &mut Command) -> Result<()> {
    let mut child = command.spawn()?;

    thread::spawn(move || {
        spawn_signal_handler();
    });

    let status = child.wait()?;

    if status.success() {
        return Ok(());
    } else {
        println!("Error code encountered: {:?}", status);
        return Err(Error::ExecError);
    }
}

fn output_exec(
    configs: &Vec<Config>,
    cmd_args: &mut Vec<&str>,
    output_non_sensitive: bool,
) -> Result<()> {
    let command = cmd_args.remove(0);
    let ssm_client = ssm::SsmClient::default();
    let mut parameters = Vec::new();
    let mut ssm = HashSet::new();

    for config in configs {
        let fields = ssm_client.get_parameters(config)?;
        extend(&mut ssm, fields);
    }

    let ssm = ssm.into_iter().collect::<Vec<_>>();
    if output_non_sensitive {
        let mut closure = move |pairs: Vec<(String, String)>| {
            for (k, v) in pairs {
                println!("{}={}", k, v);
            }
        };
        ssm.clone()
            .into_iter()
            .filter(|parameter| {
                parameter
                    .clone()
                    .type_
                    .map(|type_| match type_.as_str() {
                        "SecureString" => false,
                        _ => true,
                    })
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>()
            .export()
            .map(&mut closure);
    }

    ssm.export().map(|mut pairs| parameters.append(&mut pairs));

    let mut command = Command::new(command);

    if !parameters.is_empty() {
        command.envs(parameters);
    }

    if !cmd_args.is_empty() {
        command.args(cmd_args);
    }

    run_command(&mut command)
}

//fn output_shell(config: &Config, key: &str) -> Result<()> {
//    //let secrets_manager = secretsmanager::SecretsManagerClient::default();
//    //let secret = secrets_manager.get_secret_value(config, key)?;
//
//    if let Some(shell_config) = secret.secret_string {
//        let postgres: Postgres = serde_json::from_str(&shell_config)?;
//
//        Command::new("psql")
//            .env_clear()
//            .envs(Into::<Vec<(String, String)>>::into(postgres))
//            .spawn()
//            .map(|_| ())
//            .map_err(Into::into)
//    } else {
//        Err(Error::InvalidKey(format!("{}{}", config.as_path(), key)))
//    }
//}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let environment = matches.value_of("environment").expect("required field");
    let services = matches.value_of("service").expect("required field");
    let services = services.split(",");
    let output_non_sensitive = value_t!(matches, "output-non-sensitive", bool).unwrap_or_default();

    let mut configs = vec![];
    for service in services {
        configs.push(Config::new(environment, service));
    }

    let result = if matches.subcommand_matches("describe").is_some() {
        output_describe(&configs)
    } else if matches.subcommand_matches("stdout").is_some() {
        output_stdout(&configs)
    } else if let Some(file_matches) = matches.subcommand_matches("file") {
        let path = file_matches.value_of("path").expect("required field");

        output_file(&configs, path)
    } else if let Some(exec_matches) = matches.subcommand_matches("exec") {
        let mut cmd = exec_matches
            .values_of("cmd")
            .expect("required field")
            .collect();

        output_exec(&configs, &mut cmd, output_non_sensitive)
    }
    //else if let Some(shell_matches) = matches.subcommand_matches("shell") {
    //    let key = shell_matches.value_of("key").expect("required field");

    //    output_shell(&config, key)
    //}
    else {
        unreachable!()
    };

    result.unwrap()
}
