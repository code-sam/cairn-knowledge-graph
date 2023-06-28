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
use stacked_linear_algebra_graph::error::{
    GraphComputingError as LinearAlgebraGraphComputingError,
    GraphComputingErrorType as LinearAlgebraGraphComputingErrorType,
};
use stacked_linear_algebra_graph::graphblas_sparse_linear_algebra;

#[derive(Debug)]
pub enum KnowledgeGraphError {
    SystemError(SystemError),
    LogicError(LogicError),
    UserError(UserError),
    OtherError(OtherError),
}

#[derive(Clone, Debug, PartialEq)]
pub enum KnowledgeGraphErrorType {
    SystemErrorType(SystemErrorType),
    LogicErrorType(LogicErrorType),
    UserErrorType(UserErrorType),
    OtherErrorType(OtherErrorType),
}

impl error::Error for KnowledgeGraphError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            KnowledgeGraphError::SystemError(error) => Some(error),
            KnowledgeGraphError::LogicError(error) => Some(error),
            KnowledgeGraphError::UserError(error) => Some(error),
            KnowledgeGraphError::OtherError(error) => Some(error),
        }
    }
}

impl KnowledgeGraphError {
    pub fn error_type(&self) -> KnowledgeGraphErrorType {
        match self {
            KnowledgeGraphError::SystemError(error) => {
                KnowledgeGraphErrorType::SystemErrorType(error.error_type())
            }
            KnowledgeGraphError::LogicError(error) => {
                KnowledgeGraphErrorType::LogicErrorType(error.error_type())
            }
            KnowledgeGraphError::UserError(error) => {
                KnowledgeGraphErrorType::UserErrorType(error.error_type())
            }
            KnowledgeGraphError::OtherError(error) => {
                KnowledgeGraphErrorType::OtherErrorType(error.error_type())
            }
        }
    }
}

impl fmt::Display for KnowledgeGraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.source().unwrap());
        Ok(())
    }
}

impl From<SystemError> for KnowledgeGraphError {
    fn from(error: SystemError) -> Self {
        KnowledgeGraphError::SystemError(error)
    }
}

impl From<LogicError> for KnowledgeGraphError {
    fn from(error: LogicError) -> Self {
        KnowledgeGraphError::LogicError(error)
    }
}

impl From<UserError> for KnowledgeGraphError {
    fn from(error: UserError) -> Self {
        KnowledgeGraphError::UserError(error)
    }
}

impl From<std::fmt::Error> for KnowledgeGraphError {
    fn from(error: std::fmt::Error) -> Self {
        KnowledgeGraphError::OtherError(error.into())
    }
}

impl From<KnowledgeGraphError> for std::fmt::Error {
    fn from(_error: KnowledgeGraphError) -> Self {
        std::fmt::Error {}
    }
}

impl From<SparseLinearAlgebraError> for KnowledgeGraphError {
    fn from(error: SparseLinearAlgebraError) -> Self {
        match error.error_type() {
            SparseLinearAlgebraErrorType::LogicErrorType(_) => Self::LogicError(error.into()),
            SparseLinearAlgebraErrorType::OtherErrorType(_) => Self::OtherError(error.into()),
            SparseLinearAlgebraErrorType::SystemErrorType(_) => Self::SystemError(error.into()),
        }
    }
}

impl From<LinearAlgebraGraphComputingError> for KnowledgeGraphError {
    fn from(error: LinearAlgebraGraphComputingError) -> Self {
        match error.error_type() {
            LinearAlgebraGraphComputingErrorType::LogicErrorType(_) => {
                Self::LogicError(error.into())
            }
            LinearAlgebraGraphComputingErrorType::OtherErrorType(_) => {
                Self::OtherError(error.into())
            }
            LinearAlgebraGraphComputingErrorType::SystemErrorType(_) => {
                Self::SystemError(error.into())
            }
            LinearAlgebraGraphComputingErrorType::UserErrorType(_) => Self::UserError(error.into()),
        }
    }
}
