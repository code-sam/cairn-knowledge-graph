use std::fmt::Display;

use stacked_linear_algebra_graph::{
    graph::{self, value_type::GetValueTypeIdentifier},
    graphblas_sparse_linear_algebra::collections::sparse_vector::operations::{
        GetVectorElementValueTyped, SetVectorElementTyped,
    },
    operators::add::{AddEdgeType, AddVertex, AddVertexType},
};

use crate::{
    error::KnowledgeGraphError,
    graph::{
        AsStackedLinearAlgebraGraph, EntityTypeIndex, GetRawEntityDefinition,
        GetRawRelationshipTypeDefinition, KnowledgeGraph, ValueType, EntityIndex,
    },
};

pub trait AddRawEntity<T: ValueType> {
    fn add_entity(
        &mut self,
        entity_type: &EntityTypeIndex,
        value: T,
    ) -> Result<EntityIndex, KnowledgeGraphError>;
}

impl<T> AddRawEntity<T> for KnowledgeGraph
where
    T: ValueType + SetVectorElementTyped<T> + Copy,
{
    fn add_entity(
        &mut self,
        entity_type: &EntityTypeIndex,
        value: T,
    ) -> Result<EntityIndex, KnowledgeGraphError> {
        Ok(self
            .stacked_linear_algebra_graph_mut_ref()
            .add_vertex(entity_type, value)?)
    }
}
