extern crate chrono;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
extern crate rusoto_core;
extern crate rusoto_secretsmanager;
extern crate rusoto_ssm;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::{fs::File, io::prelude::*, path::PathBuf, process::Command};

use clap::App;

mod config;
mod error;
mod output;
mod secretsmanager;
mod ssm;
mod types;

use config::Config;
use error::Error;
use types::Result;
use output::{Exportable, Postgres, Printable};

fn output_describe(config: &Config) -> Result<()> {
    let ssm = ssm::SsmClient::default();
    let ssm = ssm.describe_parameters(config)?;
    let secrets_manager = secretsmanager::SecretsManagerClient::default();
    let secrets_manager = secrets_manager.list_secrets(config)?;

    // TODO fix this print format
    ssm.get_table().printstd();
    secrets_manager.get_table().printstd();

    Ok(())
}

fn output_stdout(config: &Config) -> Result<()> {
    let ssm = ssm::SsmClient::default();
    let ssm = ssm.get_parameters(config)?;
    let secrets_manager = secretsmanager::SecretsManagerClient::default();
    let secrets_manager = secrets_manager.get_secret_values(config)?;

    let mut closure = move |pairs: Vec<(String, String)>| {
        for (k, v) in pairs {
            println!("{}={}", k, v);
        }
    };

    ssm.export().map(&mut closure);
    secrets_manager.export().map(&mut closure);

    Ok(())
}

fn output_file<S>(config: &Config, path: S) -> Result<()>
where
    S: Into<PathBuf>,
{
    let path = path.into();
    let ssm = ssm::SsmClient::default();
    let ssm = ssm.get_parameters(config)?;
    let secrets_manager = secretsmanager::SecretsManagerClient::default();
    let secrets_manager = secrets_manager.get_secret_values(config)?;

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

    ssm.export().map(&mut closure);
    secrets_manager.export().map(&mut closure);

    Ok(())
}

fn output_exec(config: &Config, cmd: &str) -> Result<()> {
    let mut parameters = Vec::new();
    let ssm = ssm::SsmClient::default();
    let ssm = ssm.get_parameters(config)?;
    let secrets_manager = secretsmanager::SecretsManagerClient::default();
    let secrets_manager = secrets_manager.get_secret_values(config)?;

    ssm.export().map(|mut pairs| parameters.append(&mut pairs));
    secrets_manager
        .export()
        .map(|mut pairs| parameters.append(&mut pairs));

    if parameters.is_empty() {
        Command::new(cmd)
            .env_clear()
            .spawn()
            .map(|_| ())
            .map_err(Into::into)
    } else {
        Command::new(cmd)
            .env_clear()
            .envs(parameters)
            .spawn()
            .map(|_| ())
            .map_err(Into::into)
    }
}

fn output_shell(config: &Config, key: &str) -> Result<()> {
    let secrets_manager = secretsmanager::SecretsManagerClient::default();
    let secret = secrets_manager.get_secret_value(config, key)?;

    if let Some(shell_config) = secret.secret_string {
        let postgres: Postgres = serde_json::from_str(&shell_config)?;

        Command::new("psql")
            .env_clear()
            .envs(Into::<Vec<(String, String)>>::into(postgres))
            .spawn()
            .map(|_| ())
            .map_err(Into::into)
    } else {
        Err(Error::InvalidKey(format!("{}/{}", config.as_path(), key)))
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let environment = matches.value_of("environment").expect("required field");
    let service = matches.value_of("service").expect("required field");
    let config = Config::new(environment, service);

    let result = if matches.subcommand_matches("describe").is_some() {
        output_describe(&config)
    } else if matches.subcommand_matches("stdout").is_some() {
        output_stdout(&config)
    } else if let Some(file_matches) = matches.subcommand_matches("file") {
        let path = file_matches.value_of("path").expect("required field");

        output_file(&config, path)
    } else if let Some(exec_matches) = matches.subcommand_matches("exec") {
        let cmd = exec_matches.value_of("cmd").expect("required field");

        output_exec(&config, cmd)
    } else if let Some(shell_matches) = matches.subcommand_matches("shell") {
        let key = shell_matches.value_of("key").expect("required field");

        output_shell(&config, key)
    } else {
        unreachable!()
    };

    if let Err(e) = result {
        println!("{:?}", e)
    }
}
