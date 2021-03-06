use crate::error::GraphComputingError;
use crate::error::{UserError, UserErrorType};

use crate::graph::graph::Graph;
use crate::graph::vertex::{VertexIndex, VertexKey, VertexValue};

pub trait ReadVertexValue {
    fn is_valid_vertex_key(&self, vertex_key: &VertexKey) -> bool;
    fn is_valid_vertex_index(&self, vertex_key: &VertexIndex) -> Result<bool, GraphComputingError>;
    fn vertex_value(&self, vertex_key: &VertexKey) -> Result<&VertexValue, GraphComputingError>;
    fn vertex_value_by_index(
        &self,
        vertex_index: VertexIndex,
    ) -> Result<&VertexValue, GraphComputingError>;
}

impl ReadVertexValue for Graph {
    fn vertex_value(&self, vertex_key: &VertexKey) -> Result<&VertexValue, GraphComputingError> {
        match self.vertex_key_to_vertex_index_map_ref().get(vertex_key) {
            None => Err(UserError::new(
                UserErrorType::VertexKeyNotFound,
                format!("No vertex found for key \"{}\")", vertex_key),
                None,
            )
            .into()),
            Some(&vertex_index) => self.vertex_value_by_index(vertex_index),
        }
    }

    fn vertex_value_by_index(
        &self,
        vertex_index: VertexIndex,
    ) -> Result<&VertexValue, GraphComputingError> {
        let vertex = self.vertex_store_ref().get_ref(vertex_index)?;
        Ok(vertex.value_ref())
    }

    fn is_valid_vertex_key(&self, vertex_key: &VertexKey) -> bool {
        match self.vertex_key_to_vertex_index_map_ref().get(vertex_key) {
            None => false,
            Some(_) => true,
        }
    }

    fn is_valid_vertex_index(
        &self,
        vertex_index: &VertexIndex,
    ) -> Result<bool, GraphComputingError> {
        self.vertex_store_ref().is_valid_index(vertex_index)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // use crate::graph::vertex::VertexValue;

    // TODO
    // #[test]
    // fn new_graph() {
    //     let graph = Graph::new(10, 20);
    // }
}
