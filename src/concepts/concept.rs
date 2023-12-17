use stacked_linear_algebra_graph::graph::graph::VertexIndex;

use crate::{error::KnowledgeGraphError, graph::KnowledgeGraph};

pub type ConceptIndex = VertexIndex;

pub struct Concept {}

pub struct ConcepEnabler {}

pub struct EnabledConceptConcept {}

pub trait AddConcept<Graph, ConceptConfiguration> {
    fn add_concept(
        &mut self,
        graph: &mut Graph,
    ) -> Result<ConceptConfiguration, KnowledgeGraphError>;
}

// pub trait GetConceptConfiguration {
//     fn concept_configuration()
// }

pub trait CheckConceptSupport<Graph> {
    fn is_supported_by(&self, graph: &Graph) -> Result<bool, crate::error::KnowledgeGraphError>;

    fn try_supported_by(&self, graph: &Graph) -> Result<(), crate::error::KnowledgeGraphError>;
}
