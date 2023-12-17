use stacked_linear_algebra_graph::graph::graph::{VertexIndex, VertexTypeIndex};

use crate::{error::KnowledgeGraphError, graph::KnowledgeGraph};

pub trait EnableCharacterConcept {
    fn enable(&mut self) -> Result<CharacterConceptConfiguration, KnowledgeGraphError>;
}

impl EnableCharacterConcept for KnowledgeGraph {
    fn enable(&mut self) -> Result<CharacterConceptConfiguration, KnowledgeGraphError> {
        todo!()
    }
}

pub trait CharacterConceptConfigurationStore {}

pub struct CharacterConceptConfiguration {
    entityVertexFilterIndex: VertexTypeIndex,

    conceptVertexIndex: VertexIndex,
}
