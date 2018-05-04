use rusoto_secretsmanager::{GetSecretValueError, ListSecretsError};
use rusoto_ssm::{DescribeParametersError, GetParametersByPathError};

#[derive(Debug)]
pub enum Error {
    GetSecretValueError(GetSecretValueError),
    ListSecretsError(ListSecretsError),
    DescribeParametersError(DescribeParametersError),
    GetParametersByPathError(GetParametersByPathError),
}

impl From<GetSecretValueError> for Error {
    fn from(e: GetSecretValueError) -> Self {
        Error::GetSecretValueError(e)
    }
}

impl From<ListSecretsError> for Error {
    fn from(e: ListSecretsError) -> Self {
        Error::ListSecretsError(e)
    }
}

impl From<DescribeParametersError> for Error {
    fn from(e: DescribeParametersError) -> Self {
        Error::DescribeParametersError(e)
    }
}

impl From<GetParametersByPathError> for Error {
    fn from(e: GetParametersByPathError) -> Self {
        Error::GetParametersByPathError(e)
    }
}
