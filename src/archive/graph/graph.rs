use stacked_linear_algebra_graph::graph::graph::Graph as StackedLinearAlgebraGraph;
use stacked_linear_algebra_graph::operators::add::{AddVertex, AddVertexType};

use crate::concepts::cairn_lang::cairn_lang::CAIRN_LANG_NAMESPACE_KEY;
use crate::concepts::primitives::configuration::{
    GetKnowledgeGraphPrimitivesConfiguration, KnowledgeGraphPrimitivesConfiguration,
};
use crate::concepts::primitives::primitives::ImplementPrimitiveConcepts;
use crate::error::KnowledgeGraphError;

use super::CapacityConfiguration;

// trait aliases are experimental
// see issue #41517 <https://github.com/rust-lang/rust/issues/41517> for more information
// pub trait AddEntity = AddVertex;
// pub trait AddEntityType = AddVertexType;

pub const DEFAULT_INITIAL_ENTITY_CAPACITY: usize = 256;
pub const DEFAULT_INITIAL_ENTITY_TYPE_CAPACITY: usize = 256;
pub const DEFAULT_INITIAL_RELATIONSHIP_TYPE_CAPACITY: usize = 256;

pub struct KnowledgeGraph {
    stacked_linear_algebra_graph: StackedLinearAlgebraGraph,
    // TODO: architecture for performant and easy-to-use (primitive) concepts
    // primitives_configuration: KnowledgeGraphPrimitivesConfiguration,
}

impl KnowledgeGraph {
    pub fn new(
        initial_capacity_configuration: &CapacityConfiguration,
        primitives_configuration: &impl GetKnowledgeGraphPrimitivesConfiguration,
    ) -> Result<Self, KnowledgeGraphError> {
        let mut stacked_linear_algebra_graph = StackedLinearAlgebraGraph::with_initial_capacity(
            initial_capacity_configuration.entity_type_capacity_ref(),
            initial_capacity_configuration.entity_capacity_ref(),
            initial_capacity_configuration.relationship_type_capacity_ref(),
        )?;

        let mut knowledge_graph = Self {
            stacked_linear_algebra_graph,
        };
        knowledge_graph.implement_primitive_concepts(primitives_configuration)?;

        Ok(knowledge_graph)
    }

    pub fn new_default() -> Result<Self, KnowledgeGraphError> {
        let initial_capacity_configuration = CapacityConfiguration::new(
            DEFAULT_INITIAL_ENTITY_CAPACITY,
            DEFAULT_INITIAL_ENTITY_TYPE_CAPACITY,
            DEFAULT_INITIAL_RELATIONSHIP_TYPE_CAPACITY,
        );

        todo!()
        // Self::new(&initial_capacity_configuration, )
    }
}

pub(crate) trait AsStackedLinearAlgebraGraph {
    fn stacked_linear_algebra_graph_ref(&self) -> &StackedLinearAlgebraGraph;
    fn stacked_linear_algebra_graph_mut_ref(&mut self) -> &mut StackedLinearAlgebraGraph;

    // fn cairn_lang_primitives_configuration_ref(&self) -> &KnowledgeGraphPrimitivesConfiguration;
}

impl AsStackedLinearAlgebraGraph for KnowledgeGraph {
    fn stacked_linear_algebra_graph_ref(&self) -> &StackedLinearAlgebraGraph {
        &self.stacked_linear_algebra_graph
    }

    fn stacked_linear_algebra_graph_mut_ref(&mut self) -> &mut StackedLinearAlgebraGraph {
        &mut self.stacked_linear_algebra_graph
    }

    // fn cairn_lang_primitives_configuration_ref(&self) -> &KnowledgeGraphPrimitivesConfiguration {
    //     &self.primitives_configuration
    // }
}
