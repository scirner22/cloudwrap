use rusoto_core::Region;
use rusoto_ssm::{DescribeParametersRequest, DescribeParametersResult, GetParametersByPathRequest,
                 GetParametersByPathResult, Parameter, ParameterMetadata, ParameterStringFilter,
                 Ssm, SsmClient as Client};

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
            key: String::from("Path"),
            option: Some(String::from("OneLevel")),
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
            let res = self.inner.describe_parameters(req).sync()?;
            res.parameters
                .clone()
                .map(|mut p| parameters.append(&mut p));
            let res = Into::<WrappedDescribeParametersResult>::into(res);

            match Into::<Option<DescribeParametersRequest>>::into(res) {
                Some(next_req) => req = next_req,
                _ => return Ok(parameters),
            }
        }
    }

    fn initial_get_request(&self, config: &Config) -> GetParametersByPathRequest {
        let mut req = GetParametersByPathRequest::default();
        req.path = format!("{}/", config.as_path());
        req.recursive = Some(true);
        req
    }

    pub fn get_parameters(&self, config: &Config) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();
        let mut req = self.initial_get_request(config);

        loop {
            let res = self.inner.get_parameters_by_path(req).sync()?;
            res.parameters
                .clone()
                .map(|mut p| parameters.append(&mut p));
            let res = Into::<WrappedGetParametersByPathResult>::into(res);

            match Into::<Option<GetParametersByPathRequest>>::into(res) {
                Some(next_req) => req = next_req,
                _ => return Ok(parameters),
            }
        }
    }
}

struct WrappedDescribeParametersResult {
    inner: DescribeParametersResult,
}

impl From<DescribeParametersResult> for WrappedDescribeParametersResult {
    fn from(res: DescribeParametersResult) -> Self {
        WrappedDescribeParametersResult { inner: res }
    }
}

impl From<WrappedDescribeParametersResult> for Option<DescribeParametersRequest> {
    fn from(res: WrappedDescribeParametersResult) -> Self {
        res.inner.next_token.and_then(|token| {
            let mut req = DescribeParametersRequest::default();
            req.next_token = Some(token);
            Some(req)
        })
    }
}

struct WrappedGetParametersByPathResult {
    inner: GetParametersByPathResult,
}

impl From<GetParametersByPathResult> for WrappedGetParametersByPathResult {
    fn from(res: GetParametersByPathResult) -> Self {
        WrappedGetParametersByPathResult { inner: res }
    }
}

impl From<WrappedGetParametersByPathResult> for Option<GetParametersByPathRequest> {
    fn from(res: WrappedGetParametersByPathResult) -> Self {
        res.inner.next_token.and_then(|token| {
            let mut req = GetParametersByPathRequest::default();
            req.next_token = Some(token);
            Some(req)
        })
    }
}
