use std::error;
use std::error::Error;
use std::fmt;

use graphblas_sparse_linear_algebra::error::{
    SparseLinearAlgebraError, SparseLinearAlgebraErrorType,
};
use stacked_linear_algebra_graph::{
    error::{
        GraphComputingError as LinearAlgebraGraphComputingError,
        GraphComputingErrorType as LinearAlgebraGraphComputingErrorType,
    },
    graphblas_sparse_linear_algebra,
};

#[derive(Debug)]
pub struct SystemError {
    error_type: SystemErrorType,
    explanation: String,
    source: Option<SystemErrorSource>,
}

#[derive(Debug)]
pub enum SystemErrorSource {
    SparseLinearAlgebra(SparseLinearAlgebraError),
    LinearAlgebraGraphComputing(LinearAlgebraGraphComputingError),
    PoisonedData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SystemErrorType {
    SparseLinearAlgebra(SparseLinearAlgebraErrorType),
    LinearAlgebraGraphComputing(LinearAlgebraGraphComputingErrorType),
    CreateGraphBlasErrorOnSuccessValue,
    KeyNotFound,
    UnsupportedGraphBlasErrorValue,
    UninitialisedContext,
    ContextAlreadyInitialized,
    PoisonedData,
    IndexOutOfBounds,
    Other,
}

impl SystemError {
    pub fn new(
        error_type: SystemErrorType,
        explanation: String,
        source: Option<SystemErrorSource>,
    ) -> Self {
        Self {
            error_type,
            explanation,
            source,
        }
    }

    pub fn error_type(&self) -> SystemErrorType {
        self.error_type.clone()
    }
    pub fn explanation(&self) -> String {
        self.explanation.clone()
    }
}

impl error::Error for SystemError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.source {
            Some(ref error) => match error {
                SystemErrorSource::SparseLinearAlgebra(error) => Some(error),
                SystemErrorSource::PoisonedData => None,
                SystemErrorSource::LinearAlgebraGraphComputing(error) => Some(error),
            },
            None => None,
        }
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error_type {
            _ => writeln!(f, "Explanation:\n{}", &self.explanation)?,
        };

        match &self.source() {
            Some(err) => writeln!(f, "Source error:\n{}", err)?,
            None => (),
        }
        Ok(())
    }
}

impl From<SparseLinearAlgebraError> for SystemError {
    fn from(error: SparseLinearAlgebraError) -> Self {
        Self {
            error_type: SystemErrorType::SparseLinearAlgebra(error.error_type()),
            explanation: String::new(),
            source: Some(SystemErrorSource::SparseLinearAlgebra(error)),
        }
    }
}

impl From<LinearAlgebraGraphComputingError> for SystemError {
    fn from(error: LinearAlgebraGraphComputingError) -> Self {
        Self {
            error_type: SystemErrorType::LinearAlgebraGraphComputing(error.error_type()),
            explanation: String::new(),
            source: Some(SystemErrorSource::LinearAlgebraGraphComputing(error)),
        }
    }
}
