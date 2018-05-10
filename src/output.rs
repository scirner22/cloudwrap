use chrono::NaiveDateTime;
use prettytable::{format, Table};
use rusoto_secretsmanager::{GetSecretValueResponse, SecretListEntry};
use rusoto_ssm::{Parameter, ParameterMetadata};

fn split_take_last(ch: char, field: Option<String>) -> String {
    field
        .and_then(|f| f.split(ch).collect::<Vec<&str>>().pop().map(Into::into))
        .unwrap_or_else(|| "".into())
}

trait Service {
    fn get_service_name(&self) -> String;
}

impl Service for Parameter {
    fn get_service_name(&self) -> String {
        split_take_last('/', self.name.clone())
    }
}

impl Service for ParameterMetadata {
    fn get_service_name(&self) -> String {
        split_take_last('/', self.name.clone())
    }
}

impl Service for SecretListEntry {
    fn get_service_name(&self) -> String {
        split_take_last('/', self.name.clone())
    }
}

impl Service for GetSecretValueResponse {
    fn get_service_name(&self) -> String {
        split_take_last('/', self.name.clone())
    }
}

pub trait Printable {
    fn get_table(&self) -> Table;
}

impl Printable for Vec<Parameter> {
    fn get_table(&self) -> Table {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["KEY", "VALUE"]);

        for p in self {
            let key = p.get_service_name();
            let value = p.value.clone().unwrap();

            table.add_row(row![key, value]);
        }

        table
    }
}

impl Printable for Vec<ParameterMetadata> {
    fn get_table(&self) -> Table {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row![c => "KEY", "VERSION", "LAST_MODIFIED_USER", "LAST_MODIFIED_DATE"]);

        for p in self {
            let key = p.get_service_name();
            let user = split_take_last('/', p.last_modified_user.clone());
            let version = p.version.unwrap_or(0);
            let date = p.last_modified_date.unwrap();
            let date = NaiveDateTime::from_timestamp(date.floor() as i64, 0);

            table.add_row(row![r => key, format!("{}", version), user, date]);
        }

        table
    }
}

impl Printable for Vec<SecretListEntry> {
    fn get_table(&self) -> Table {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row![c => "KEY", "VERSION", "LAST_MODIFIED_USER", "LAST_MODIFIED_DATE"]);

        for p in self {
            let key = p.get_service_name();
            // TODO is this the best way to display these fields?
            let user = p.rotation_enabled
                .map(|_| "lambda")
                .unwrap_or("no rotation policy");
            let version = 0;
            let date = p.last_changed_date.unwrap();
            let date = NaiveDateTime::from_timestamp(date.floor() as i64, 0);

            table.add_row(row![r => key, format!("{}", version), user, date]);
        }

        table
    }
}

impl Printable for Vec<GetSecretValueResponse> {
    fn get_table(&self) -> Table {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["KEY", "VALUE"]);

        for p in self {
            let key = p.get_service_name();
            let value = p.secret_string.clone().unwrap();

            table.add_row(row![key, value]);
        }

        table
    }
}

pub trait Exportable {
    fn export(&self) -> Option<Vec<(String, String)>>;
}

impl Exportable for Vec<Parameter> {
    fn export(&self) -> Option<Vec<(String, String)>> {
        let mut pairs = Vec::new();

        for p in self {
            let key = p.get_service_name();
            let key = key.to_uppercase().replace("-", "_");
            let value = p.value.clone().unwrap();

            pairs.push((key, value));
        }

        Some(pairs)
    }
}

impl Exportable for Vec<GetSecretValueResponse> {
    fn export(&self) -> Option<Vec<(String, String)>> {
        let mut pairs = Vec::new();

        for p in self {
            let key = p.get_service_name();
            let key = key.to_uppercase().replace("-", "_");
            let value = p.secret_string.clone().unwrap();
            // TODO String::escape_default() is currently in nightly
            let value: String = value.chars().flat_map(|c| c.escape_default()).collect();

            pairs.push((key, format!("\"{}\"", value)));
        }

        Some(pairs)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Postgres {
    host: String,
    port: i32,
    dbname: String,
    username: String,
    password: String,
    engine: String,
    db_instance_identifier: String,
}

impl From<Postgres> for Vec<(String, String)> {
    fn from(p: Postgres) -> Self {
        vec![
            ("PGHOST".into(), p.host),
            ("PGPORT".into(), p.port.to_string()),
            ("PGDATABASE".into(), p.dbname),
            ("PGUSER".into(), p.username),
            ("PGPASSWORD".into(), p.password),
        ]
    }
}
