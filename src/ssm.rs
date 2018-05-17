use rusoto_core::Region;
use rusoto_ssm::{DescribeParametersRequest, GetParametersByPathRequest, Parameter,
                 ParameterMetadata, ParameterStringFilter, Ssm, SsmClient as Client};

use config::Config;
use types::Result;

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
            inner: Client::simple(region),
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
                _ => return Ok(parameters),
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
                _ => return Ok(parameters),
            }
        }
    }
}
