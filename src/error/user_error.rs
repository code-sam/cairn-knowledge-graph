use std::error;
use std::error::Error;
use std::fmt;

use graphblas_sparse_linear_algebra::error::{
    SparseLinearAlgebraError, SparseLinearAlgebraErrorType,
};
use stacked_linear_algebra_graph::graphblas_sparse_linear_algebra;

#[derive(Debug)]
pub struct UserError {
    error_type: UserErrorType,
    explanation: String,
    source: Option<UserErrorSource>,
}

#[derive(Debug)]
pub enum UserErrorSource {
    SparseLinearAlgebra(SparseLinearAlgebraError),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserErrorType {
    SparseLinearAlgebra(SparseLinearAlgebraErrorType),
    EdgeTypeDoesNotExist,
    IndexOutOfBounds,
    VertexAlreadyExists,
    VertexKeyNotFound,
    Other,
}

impl UserError {
    pub fn new(
        error_type: UserErrorType,
        explanation: String,
        source: Option<UserErrorSource>,
    ) -> Self {
        Self {
            error_type,
            explanation,
            source,
        }
    }

    pub fn error_type(&self) -> UserErrorType {
        self.error_type.clone()
    }
    pub fn explanation(&self) -> String {
        self.explanation.clone()
    }
}

impl error::Error for UserError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.source {
            Some(ref error) => match error {
                UserErrorSource::SparseLinearAlgebra(error) => Some(error),
            },
            None => None,
        }
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error_type {
            // LogicErrorType::SparseLinearAlgebra(_err) => writeln!(f, "Context:\n{}", &self.context)?,
            _ => writeln!(f, "Context:\n{}", &self.explanation)?,
        };

        match &self.source() {
            Some(err) => writeln!(f, "Source error:\n{}", err)?,
            &None => (),
        }
        Ok(())
    }
}

impl From<SparseLinearAlgebraError> for UserError {
    fn from(error: SparseLinearAlgebraError) -> Self {
        Self {
            error_type: UserErrorType::SparseLinearAlgebra(error.error_type()),
            explanation: String::new(),
            source: Some(UserErrorSource::SparseLinearAlgebra(error)),
        }
    }
}
