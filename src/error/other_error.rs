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
pub struct OtherError {
    error_type: OtherErrorType,
    explanation: String,
    source: Option<OtherErrorSource>,
}

#[derive(Debug)]
pub enum OtherErrorSource {
    Display(std::fmt::Error),
    SparseLinearAlgebra(SparseLinearAlgebraError),
    LinearAlgebraGraphComputing(LinearAlgebraGraphComputingError),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OtherErrorType {
    Display,
    Other,
    SparseLinearAlgebra(SparseLinearAlgebraErrorType),
    LinearAlgebraGraphComputing(LinearAlgebraGraphComputingErrorType),
}

impl OtherError {
    pub fn new(
        error_type: OtherErrorType,
        explanation: String,
        source: Option<OtherErrorSource>,
    ) -> Self {
        Self {
            error_type,
            explanation,
            source,
        }
    }

    pub fn error_type(&self) -> OtherErrorType {
        self.error_type.clone()
    }
    pub fn explanation(&self) -> String {
        self.explanation.clone()
    }
}

impl error::Error for OtherError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.source {
            Some(ref error) => match error {
                OtherErrorSource::Display(error) => Some(error),
                OtherErrorSource::SparseLinearAlgebra(error) => Some(error),
                OtherErrorSource::LinearAlgebraGraphComputing(error) => Some(error),
            },
            None => None,
        }
    }
}

impl fmt::Display for OtherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error_type {
            // LogicErrorType::GraphBlas(_err) => writeln!(f, "Context:\n{}", &self.context)?,
            _ => writeln!(f, "Explanation:\n{}", &self.explanation)?,
        };

        match &self.source() {
            Some(err) => writeln!(f, "Source error:\n{}", err)?,
            &None => (),
        }
        Ok(())
    }
}

impl From<std::fmt::Error> for OtherError {
    fn from(error: std::fmt::Error) -> Self {
        Self {
            error_type: OtherErrorType::Display,
            explanation: String::new(),
            source: Some(OtherErrorSource::Display(error)),
        }
    }
}

impl From<SparseLinearAlgebraError> for OtherError {
    fn from(error: SparseLinearAlgebraError) -> Self {
        Self {
            error_type: OtherErrorType::SparseLinearAlgebra(error.error_type()),
            explanation: String::new(),
            source: Some(OtherErrorSource::SparseLinearAlgebra(error)),
        }
    }
}

impl From<LinearAlgebraGraphComputingError> for OtherError {
    fn from(error: LinearAlgebraGraphComputingError) -> Self {
        Self {
            error_type: OtherErrorType::LinearAlgebraGraphComputing(error.error_type()),
            explanation: String::new(),
            source: Some(OtherErrorSource::LinearAlgebraGraphComputing(error)),
        }
    }
}
