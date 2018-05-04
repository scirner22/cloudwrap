use chrono::NaiveDateTime;
use prettytable::{format, Table};
use rusoto_ssm::{Parameter, ParameterMetadata};
use rusoto_secretsmanager::{GetSecretValueResponse, SecretListEntry};

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

trait UserCreation {
    fn last_modified_user(&self) -> String;
}

impl UserCreation for ParameterMetadata {
    fn last_modified_user(&self) -> String {
        split_take_last('/', self.last_modified_user.clone())
    }
}

pub trait Printable {
    fn get_table(&self) -> Table;
    fn export(&self) -> Option<Vec<(String, String)>>;
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

impl Printable for Vec<ParameterMetadata> {
    fn get_table(&self) -> Table {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row![c => "KEY", "VERSION", "LAST_MODIFIED_USER", "LAST_MODIFIED_DATE"]);

        for p in self {
            let key = p.get_service_name();
            let user = p.last_modified_user();
            let version = p.version.unwrap_or(0);
            let date = p.last_modified_date.unwrap();
            let date = NaiveDateTime::from_timestamp(date.floor() as i64, 0);

            table.add_row(row![r => key, format!("{}", version), user, date]);
        }

        table
    }

    fn export(&self) -> Option<Vec<(String, String)>> {
        None
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

    fn export(&self) -> Option<Vec<(String, String)>> {
        None
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

    fn export(&self) -> Option<Vec<(String, String)>> {
        let mut pairs = Vec::new();

        for p in self {
            let key = p.get_service_name();
            let key = key.to_uppercase().replace("-", "_");
            let value = p.secret_string.clone().unwrap();

            pairs.push((key, value));
        }

        Some(pairs)
    }
}
