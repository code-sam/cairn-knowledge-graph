use graphblas_sparse_linear_algebra::value_type::ValueType;

use crate::{
    error::GraphComputingError,
    graph::{
        graph::{Graph, GraphTrait, VertexIndex, VertexTypeIndex},
        vertex::{VertexKeyRef, VertexTypeKeyRef},
        vertex_store::{vertex_operations::AddVertexKey as AddVertexKeyToVertexStore},
    },
    graph::indexer::AssignedIndexTrait,
};

pub trait AddVertexKey {
    fn add_new_vertex_key(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<VertexIndex, GraphComputingError>;
}

impl AddVertexKey for Graph {
    fn add_new_vertex_key(
        &mut self,
        vertex_key: &VertexKeyRef,
    ) -> Result<VertexIndex, GraphComputingError> {
        let assigned_index = self.vertex_store_mut_ref().add_new_vertex_key(vertex_key)?;
        match assigned_index.new_index_capacity() {
            Some(new_vertex_capacity) => {
                self.update_vertex_capacity(&new_vertex_capacity)?;
            }
            None => {}
        }
        return Ok(*assigned_index.index_ref());
    }
}
