use std::hash::{Hash, Hasher};

use rusoto_core::Region;
use rusoto_ssm::{
    DescribeParametersRequest, GetParametersByPathRequest, Parameter as RusotoParameter,
    ParameterMetadata as RusotoParameterMetadata, ParameterStringFilter, Ssm, SsmClient as Client,
};

use config::Config;
use types::Result;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Option<String>,
    pub type_: Option<String>,
    pub value: Option<String>,
    pub version: Option<i64>,
}

impl From<RusotoParameter> for Parameter {
    fn from(rusoto: RusotoParameter) -> Self {
        Parameter {
            name: rusoto.name,
            type_: rusoto.type_,
            value: rusoto.value,
            version: rusoto.version,
        }
    }
}

impl PartialEq for Parameter {
    fn eq(&self, other: &Parameter) -> bool {
        self.name == other.name
    }
}

impl Eq for Parameter {}

impl Hash for Parameter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

#[derive(Debug)]
pub struct ParameterMetadata {
    pub allowed_pattern: Option<String>,
    pub description: Option<String>,
    pub key_id: Option<String>,
    pub last_modified_date: Option<f64>,
    pub last_modified_user: Option<String>,
    pub name: Option<String>,
    pub type_: Option<String>,
    pub version: Option<i64>,
}

impl From<RusotoParameterMetadata> for ParameterMetadata {
    fn from(rusoto: RusotoParameterMetadata) -> Self {
        ParameterMetadata {
            allowed_pattern: rusoto.allowed_pattern,
            description: rusoto.description,
            key_id: rusoto.key_id,
            last_modified_date: rusoto.last_modified_date,
            last_modified_user: rusoto.last_modified_user,
            name: rusoto.name,
            type_: rusoto.type_,
            version: rusoto.version,
        }
    }
}

impl PartialEq for ParameterMetadata {
    fn eq(&self, other: &ParameterMetadata) -> bool {
        self.name == other.name
    }
}

impl Eq for ParameterMetadata {}

impl Hash for ParameterMetadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

pub struct SsmClient {
    inner: Client,
}

impl Default for SsmClient {
    fn default() -> Self {
        SsmClient::new(Region::UsEast1)
    }
}

impl SsmClient {
    pub fn new(region: Region) -> Self {
        SsmClient {
            inner: Client::new(region),
        }
    }

    fn initial_describe_request(&self, config: &Config) -> DescribeParametersRequest {
        let filter = ParameterStringFilter {
            key: String::from("Name"),
            option: Some(String::from("BeginsWith")),
            values: Some(vec![config.as_path()]),
        };
        let mut req = DescribeParametersRequest::default();
        req.parameter_filters = Some(vec![filter]);
        req
    }

    pub fn describe_parameters(&self, config: &Config) -> Result<Vec<ParameterMetadata>> {
        let mut parameters = Vec::new();
        let mut req = self.initial_describe_request(config);

        loop {
            let res = self.inner.describe_parameters(req.clone()).sync()?;
            res.parameters
                .clone()
                .map(|mut p| parameters.append(&mut p));

            match res.next_token {
                Some(next_token) => req.next_token = Some(next_token),
                _ => return Ok(parameters.into_iter().map(|p| p.into()).collect()),
            }
        }
    }

    fn initial_get_request(&self, config: &Config) -> GetParametersByPathRequest {
        let mut req = GetParametersByPathRequest::default();
        req.path = config.as_path();
        req.recursive = Some(true);
        req.with_decryption = Some(true);
        req
    }

    pub fn get_parameters(&self, config: &Config) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();
        let mut req = self.initial_get_request(config);

        loop {
            let res = self.inner.get_parameters_by_path(req.clone()).sync()?;
            res.parameters
                .clone()
                .map(|mut p| parameters.append(&mut p));

            match res.next_token {
                Some(next_token) => req.next_token = Some(next_token),
                _ => return Ok(parameters.into_iter().map(|p| p.into()).collect()),
            }
        }
    }
}
