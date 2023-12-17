use stacked_linear_algebra_graph::{operators::add::AddVertex, error::GraphComputingError, graphblas_sparse_linear_algebra::collections::sparse_vector::operations::SetVectorElementTyped};

use crate::graph::{KnowledgeGraph, ValueType, EntityTypeIndex, AsStackedLinearAlgebraGraph, EntityIndex, RelationshipTypeIndex};

pub trait AddRawRelationship<T> {
    fn apply(&mut self, relationship_type: &RelationshipTypeIndex, value: T) -> Result<EntityIndex, GraphComputingError>;
}

// impl<T: ValueType + SetVectorElementTyped<T> + Copy> AddRawRelationship<T> for KnowledgeGraph {
//     fn apply(&mut self, entity_type: &EntityTypeIndex, value: T) -> Result<EntityIndex, GraphComputingError> {
//         Ok(self.stacked_linear_algebra_graph_mut_ref().add_vertex(entity_type, value)?)
//     }
// }
