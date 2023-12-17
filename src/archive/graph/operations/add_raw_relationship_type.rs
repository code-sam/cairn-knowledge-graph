use stacked_linear_algebra_graph::{
    graph::value_type::GetValueTypeIdentifier, operators::add::AddEdgeType,
};

use crate::{
    error::KnowledgeGraphError,
    graph::{
        AsStackedLinearAlgebraGraph, EntityTypeIndex, GetRawEntityDefinition,
        GetRawRelationshipTypeDefinition, KnowledgeGraph, ValueType, RelationshipTypeIndex,
    },
};

pub trait AddRawRelationshipType<T: ValueType> {
    fn add_relationship_type(
        &mut self,
        type_definition: &impl GetRawRelationshipTypeDefinition,
    ) -> Result<RelationshipTypeIndex, KnowledgeGraphError>;
}

impl<T: ValueType + GetValueTypeIdentifier> AddRawRelationshipType<T> for KnowledgeGraph {
    fn add_relationship_type(
        &mut self,
        type_definition: &impl GetRawRelationshipTypeDefinition,
    ) -> Result<RelationshipTypeIndex, KnowledgeGraphError> {
        Ok(AddEdgeType::<T>::apply(
            self.stacked_linear_algebra_graph_mut_ref(),
        )?)
    }
}
