use rusoto_core::Region;
use rusoto_secretsmanager::{GetSecretValueRequest, GetSecretValueResponse, ListSecretsRequest,
                            ListSecretsResponse, SecretListEntry, SecretsManager,
                            SecretsManagerClient as Client};

use config::Config;
use error::Error;
use types::Result;

pub struct SecretsManagerClient {
    inner: Client,
}

impl Default for SecretsManagerClient {
    fn default() -> Self {
        SecretsManagerClient::new(Region::UsEast1)
    }
}

impl SecretsManagerClient {
    pub fn new(region: Region) -> Self {
        SecretsManagerClient {
            inner: Client::simple(region),
        }
    }

    pub fn list_secrets(&self, config: &Config) -> Result<Vec<SecretListEntry>> {
        let mut secrets = Vec::new();
        let mut req = ListSecretsRequest::default();

        loop {
            let res = self.inner.list_secrets(req).sync()?;
            res.secret_list.clone().map(|mut s| secrets.append(&mut s));
            let res = Into::<WrappedListSecretsResponse>::into(res);

            match Into::<Option<ListSecretsRequest>>::into(res) {
                Some(next_req) => req = next_req,
                _ => {
                    let secrets = secrets
                        .into_iter()
                        .filter(|s| s.deleted_date.is_none())
                        .filter(|s| {
                            s.name
                                .clone()
                                .unwrap_or_else(|| "".into())
                                .starts_with(&format!("{}/", config.as_path()))
                        })
                        .collect();

                    return Ok(secrets);
                }
            }
        }
    }

    pub fn get_secret_value(&self, config: &Config, key: &str) -> Result<GetSecretValueResponse> {
        let full_key = format!("{}/{}", config.as_path(), key);
        let secrets = self.list_secrets(config)?;

        // TODO cleanup
        for secret in secrets {
            if secret.name == Some(full_key.clone()) {
                let mut req = GetSecretValueRequest::default();
                req.secret_id = secret.arn.unwrap();
                let res = self.inner.get_secret_value(req).sync()?;

                return Ok(res);
            }
        }

        Err(Error::InvalidKey(full_key))
    }

    pub fn get_secret_values(&self, config: &Config) -> Result<Vec<GetSecretValueResponse>> {
        let mut secret_values = Vec::new();
        let secrets = self.list_secrets(config)?;

        for secret in secrets {
            let mut req = GetSecretValueRequest::default();
            req.secret_id = secret.arn.unwrap();
            let res = self.inner.get_secret_value(req).sync()?;
            secret_values.push(res);
        }

        Ok(secret_values)
    }
}

struct WrappedListSecretsResponse {
    inner: ListSecretsResponse,
}

impl From<ListSecretsResponse> for WrappedListSecretsResponse {
    fn from(res: ListSecretsResponse) -> Self {
        WrappedListSecretsResponse { inner: res }
    }
}

impl From<WrappedListSecretsResponse> for Option<ListSecretsRequest> {
    fn from(res: WrappedListSecretsResponse) -> Self {
        res.inner.next_token.and_then(|token| {
            let mut req = ListSecretsRequest::default();
            req.next_token = Some(token);
            Some(req)
        })
    }
}
