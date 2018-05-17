#[derive(Debug)]
pub struct Config {
    environment: String,
    service: String,
}

impl Config {
    pub fn new<S>(environment: S, service: S) -> Self
    where
        S: Into<String>,
    {
        Config {
            environment: environment.into(),
            service: service.into(),
        }
    }

    pub fn as_path(&self) -> String {
        format!("/{}/{}/", self.environment, self.service)
    }
}
