extern crate chrono;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
extern crate rusoto_core;
//extern crate rusoto_secretsmanager;
extern crate rusoto_ssm;

use std::{fs::File, io::prelude::*, path::PathBuf, process::Command};

use clap::App;

mod secretsmanager;
mod ssm;
mod config;
mod output;

use config::Config;
use output::Printable;

type Parameters = Box<Printable>;

#[derive(Debug)]
enum Output {
    Describe,
    Stdout,
    File(PathBuf),
    Exec(String),
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let environment = matches.value_of("environment").expect("required field");
    let service = matches.value_of("service").expect("required field");
    let config = Config::new(environment, service);

    let (output, parameters): (_, Parameters) = if let Some(_) = matches.subcommand_matches("describe") {
        let ssm = ssm::SsmClient::default();
        let parameters = ssm.describe_parameters(&config).unwrap();
        (Output::Describe, Box::new(parameters))
    } else if let Some(_) = matches.subcommand_matches("stdout") {
        let ssm = ssm::SsmClient::default();
        let parameters = ssm.get_parameters(&config).unwrap();
        (Output::Stdout, Box::new(parameters))
    } else if let Some(file_matches) = matches.subcommand_matches("file") {
        let path = file_matches.value_of("path").expect("required field");
        let ssm = ssm::SsmClient::default();
        let parameters = ssm.get_parameters(&config).unwrap();
        (Output::File(path.into()), Box::new(parameters))
    } else if let Some(exec_matches) = matches.subcommand_matches("exec") {
        let cmd = exec_matches.value_of("cmd").expect("required field");
        let ssm = ssm::SsmClient::default();
        let parameters = ssm.get_parameters(&config).unwrap();
        (Output::Exec(cmd.into()), Box::new(parameters))
    } else {
        // TODO this isn't really unreachable, figure out way to guarantee that in clap
        unreachable!()
    };

    match output {
        Output::File(path) => {
            path.parent().map(|p| if !p.exists() { panic!(format!("{:?} does not exist", p)) });
            let mut file = File::create(path).expect("opening file");

            parameters.export().map(|pairs| {
                for (k, v) in pairs {
                    file.write_all(format!("export {}={}\n", k, v).as_bytes())
                        .expect("writing to file");
                }
            });
        },
        Output::Exec(cmd) => {
            match parameters.export() {
                Some(parameters) => {
                    Command::new(&cmd)
                        .env_clear()
                        .envs(parameters)
                        .spawn()
                        .expect(&format!("failed to start {}", cmd));
                },
                None => {
                    Command::new(&cmd)
                        .env_clear()
                        .spawn()
                        .expect(&format!("failed to start {}", cmd));
                },
            }
        },
        _ => parameters.get_table().printstd(),
    }
}
