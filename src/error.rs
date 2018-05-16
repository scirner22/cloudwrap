use std::io::Error as IoError;

use rusoto_secretsmanager::{GetSecretValueError, ListSecretsError};
use rusoto_ssm::{DescribeParametersError, GetParametersByPathError};
use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    ExecError,
    GetSecretValueError(GetSecretValueError),
    ListSecretsError(ListSecretsError),
    DescribeParametersError(DescribeParametersError),
    GetParametersByPathError(GetParametersByPathError),
    InvalidKey(String),
    IoError(IoError),
    ParseError(JsonError),
}

impl From<DescribeParametersError> for Error {
    fn from(e: DescribeParametersError) -> Self {
        Error::DescribeParametersError(e)
    }
}

impl From<GetSecretValueError> for Error {
    fn from(e: GetSecretValueError) -> Self {
        Error::GetSecretValueError(e)
    }
}

impl From<GetParametersByPathError> for Error {
    fn from(e: GetParametersByPathError) -> Self {
        Error::GetParametersByPathError(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::IoError(e)
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Error::ParseError(e)
    }
}

impl From<ListSecretsError> for Error {
    fn from(e: ListSecretsError) -> Self {
        Error::ListSecretsError(e)
    }
}
