use std::io::Error as IoError;

use rusoto_core::RusotoError;
use rusoto_secretsmanager::{GetSecretValueError, ListSecretsError};
use rusoto_ssm::{DescribeParametersError, GetParametersByPathError};
use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    ExecError,
    GetSecretValueError(RusotoError<GetSecretValueError>),
    ListSecretsError(RusotoError<ListSecretsError>),
    DescribeParametersError(RusotoError<DescribeParametersError>),
    GetParametersByPathError(RusotoError<GetParametersByPathError>),
    RusotoUnknownError(u16, String),
    InvalidKey(String),
    IoError(IoError),
    ParseError(JsonError),
}

impl From<RusotoError<DescribeParametersError>> for Error {
    fn from(e: RusotoError<DescribeParametersError>) -> Self {
        Error::DescribeParametersError(e)
    }
}

impl From<RusotoError<GetSecretValueError>> for Error {
    fn from(e: RusotoError<GetSecretValueError>) -> Self {
        Error::GetSecretValueError(e)
    }
}

impl From<RusotoError<GetParametersByPathError>> for Error {
    fn from(e: RusotoError<GetParametersByPathError>) -> Self {
        match e {
            // Unknown errors do not show the actual readable response
            // from AWS by default
            RusotoError::Unknown(buffered_response) => Error::RusotoUnknownError(
                buffered_response.status.as_u16(),
                buffered_response.body_as_str().to_string(),
            ),
            _ => Error::GetParametersByPathError(e),
        }
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

impl From<RusotoError<ListSecretsError>> for Error {
    fn from(e: RusotoError<ListSecretsError>) -> Self {
        Error::ListSecretsError(e)
    }
}
