use std::marker::PhantomData;

use stacked_linear_algebra_graph::graph::edge::EdgeTypeIndex;
use stacked_linear_algebra_graph::graph::graph::Graph;
use stacked_linear_algebra_graph::operators::delete::DeleteVertexValue;
use stacked_linear_algebra_graph::operators::{add::AddVertexType, drop::DropVertexType};

use crate::concepts::concept::{AddConcept, CheckConceptSupport};
use crate::error::KnowledgeGraphError;
use crate::graph::operations::{AddRawRelationship, AddRawEntityType};
use crate::graph::raw_entity::EntityIndex;
use crate::graph::{
    GetRawRelationshipTypeDefinition, ValueType, ValueTypeIdentifier,
};

use super::entity_type_index::EntityTypeIndex;

pub trait IndexedEntityType {
    fn type_index_ref(&self) -> &EntityTypeIndex;
    fn entity_index_ref(&self) -> &EntityIndex;
}

pub trait AddEntityType<T: ValueType> {
    fn add_to_graph(
        &self,
        graph: &mut (impl AddVertexType<T>),
    ) -> Result<EntityTypeIndex, KnowledgeGraphError>;
}

pub trait DropSelf {
    fn drop(
        &self,
        graph: &mut (impl DropVertexType + DeleteVertexValue),
    ) -> Result<(), KnowledgeGraphError>;
}

pub trait DropEntityType {
    fn drop_by_index(
        graph: &mut (impl DropVertexType),
        index: &EntityTypeIndex,
    ) -> Result<(), KnowledgeGraphError>;
}

pub struct RawEntityTypeConceptDefinition<T: ValueType> {
    value_type: PhantomData<T>,
    definition_index: EdgeTypeIndex,
}

// impl<T: ValueType> RawEntityTypeConceptDefinition<T> {
//     pub fn new() -> Self {
//         Self {
//             value_type: PhantomData,
//             definition_index: 
//         }
//     }
// }

// impl<T: ValueType> GetRawRelationshipTypeDefinition for RawEntityTypeConceptDefinition<T> {
//     fn key_ref(&self) -> &RelationshipTypeKeyRef {
//         &self.definition_key.as_str()
//     }
// }

pub struct RawEntityTypeConceptConfiguration {}

pub struct ModelledRawEntityConcept {}

impl<Graph> CheckConceptSupport<Graph> for RawEntityTypeConceptConfiguration {
    fn is_supported_by(&self, graph: &Graph) -> Result<bool, crate::error::KnowledgeGraphError> {
        todo!()
    }

    fn try_supported_by(&self, graph: &Graph) -> Result<(), crate::error::KnowledgeGraphError> {
        todo!()
    }
}

impl<Graph, T> AddConcept<Graph, RawEntityTypeConceptConfiguration>
    for RawEntityTypeConceptDefinition<T>
where
    T: ValueType,
    Graph: AddRawEntityType<T>,
    Graph: AddRawRelationship<T>,
{
    fn add_concept(
        &mut self,
        graph: &mut Graph,
    ) -> Result<RawEntityTypeConceptConfiguration, KnowledgeGraphError> {
        // let entity_type_index = graph.apply(self)?;
        // let entity_index = graph.add_new_raw_entity(self)?;

        let concept_configuration = RawEntityTypeConceptConfiguration {};
        return Ok(concept_configuration);
    }
}
