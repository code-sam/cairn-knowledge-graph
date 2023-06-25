use std::error;
use std::error::Error;
use std::fmt;

use super::logic_error::{LogicError, LogicErrorType};
use super::other_error::{OtherError, OtherErrorType};
use super::system_error::{SystemError, SystemErrorType};
use super::user_error::{UserError, UserErrorType};

use graphblas_sparse_linear_algebra::error::{
    SparseLinearAlgebraError, SparseLinearAlgebraErrorType,
};
use stacked_linear_algebra_graph::graphblas_sparse_linear_algebra;

#[derive(Debug)]
pub enum GraphComputingError {
    SystemError(SystemError),
    LogicError(LogicError),
    UserError(UserError),
    OtherError(OtherError),
}

#[derive(Clone, Debug, PartialEq)]
pub enum GraphComputingErrorType {
    SystemErrorType(SystemErrorType),
    LogicErrorType(LogicErrorType),
    UserErrorType(UserErrorType),
    OtherErrorType(OtherErrorType),
}

impl error::Error for GraphComputingError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            GraphComputingError::SystemError(error) => Some(error),
            GraphComputingError::LogicError(error) => Some(error),
            GraphComputingError::UserError(error) => Some(error),
            GraphComputingError::OtherError(error) => Some(error),
        }
    }
}

impl GraphComputingError {
    pub fn error_type(&self) -> GraphComputingErrorType {
        match self {
            GraphComputingError::SystemError(error) => {
                GraphComputingErrorType::SystemErrorType(error.error_type())
            }
            GraphComputingError::LogicError(error) => {
                GraphComputingErrorType::LogicErrorType(error.error_type())
            }
            GraphComputingError::UserError(error) => {
                GraphComputingErrorType::UserErrorType(error.error_type())
            }
            GraphComputingError::OtherError(error) => {
                GraphComputingErrorType::OtherErrorType(error.error_type())
            }
        }
    }
}

impl fmt::Display for GraphComputingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.source().unwrap());
        Ok(())
    }
}

impl From<SystemError> for GraphComputingError {
    fn from(error: SystemError) -> Self {
        GraphComputingError::SystemError(error)
    }
}

impl From<LogicError> for GraphComputingError {
    fn from(error: LogicError) -> Self {
        GraphComputingError::LogicError(error)
    }
}

impl From<UserError> for GraphComputingError {
    fn from(error: UserError) -> Self {
        GraphComputingError::UserError(error)
    }
}

impl From<std::fmt::Error> for GraphComputingError {
    fn from(error: std::fmt::Error) -> Self {
        GraphComputingError::OtherError(error.into())
    }
}

impl From<GraphComputingError> for std::fmt::Error {
    fn from(_error: GraphComputingError) -> Self {
        std::fmt::Error {}
    }
}

impl From<SparseLinearAlgebraError> for GraphComputingError {
    fn from(error: SparseLinearAlgebraError) -> Self {
        match error.error_type() {
            SparseLinearAlgebraErrorType::LogicErrorType(_) => Self::LogicError(error.into()),
            SparseLinearAlgebraErrorType::OtherErrorType(_) => Self::OtherError(error.into()),
            SparseLinearAlgebraErrorType::SystemErrorType(_) => Self::SystemError(error.into()),
        }
    }
}
