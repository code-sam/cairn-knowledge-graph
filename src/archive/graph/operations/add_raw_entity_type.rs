use stacked_linear_algebra_graph::{
    graph::value_type::GetValueTypeIdentifier,
    operators::add::{AddEdgeType, AddVertexType},
};

use crate::{
    error::KnowledgeGraphError,
    graph::{
        AsStackedLinearAlgebraGraph, EntityTypeIndex, GetRawEntityDefinition,
        GetRawRelationshipTypeDefinition, KnowledgeGraph, ValueType,
    },
};

pub trait AddRawEntityType<T: ValueType> {
    fn apply(&mut self) -> Result<EntityTypeIndex, KnowledgeGraphError>;
}

impl<T: ValueType + GetValueTypeIdentifier> AddRawEntityType<T> for KnowledgeGraph {
    fn apply(&mut self) -> Result<EntityTypeIndex, KnowledgeGraphError> {
        Ok(AddVertexType::<T>::apply(
            self.stacked_linear_algebra_graph_mut_ref(),
        )?)
    }
}
