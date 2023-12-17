use stacked_linear_algebra_graph::{
    graph::graph::VertexIndex,
    operators::{add::AddVertex, delete::DeleteVertexValue},
};

use crate::error::KnowledgeGraphError;

use super::entity_index::EntityIndex;

// pub trait AddSelf<T> {
//     // Ideally, the return type would be impl IndexedEntity, however this is not yet supported in Rust.
//     // Providing a concrete implementation like in an InOut argument ascommon in C is thus a work-around.
//     // see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for support status
//     // fn add_to_graph_ideal(graph: &mut impl AddVertex<T>) -> Result<impl IndexedEntity, KnowledgeGraphError>;
//     fn add_to_graph(
//         &self,
//         graph: &mut (impl AddVertex<T> + AddVertex<bool>),
//     ) -> Result<EntityIndex, KnowledgeGraphError>;
// }

pub trait IndexedRawEntity {
    fn index_ref(&self) -> EntityIndex;

    // // TODO: deleting a Vertex only requires a key or an index, but not both. Defining
    // fn delete(&self, graph: &mut impl DeleteVertex) -> Result<(), KnowledgeGraphError>;

    // TODO: operation not yet supported by underlying graph
    // NOTE: think about the potential consequences of renaming to IndexedEntities
    // fn rename_key(&self, &mut impl , key: &VertexKeyRef) -> Result<(), KnowledgeGraphError>;
}

pub trait DeleteSelf {
    fn delete(&self, graph: &mut impl DeleteVertexValue) -> Result<(), KnowledgeGraphError>;
}

pub trait DeleteEntity {
    fn delete_by_index(&mut self, index: &EntityIndex) -> Result<(), KnowledgeGraphError>;
}

// An entity is an indexed instance/implementation of a concept.

// An entity has a:
//  - namespace
//  - concept
//  - ...

pub trait GetRawEntityDefinition<T> {
    fn value_ref(&self) -> &T;
    fn index_ref(&self) -> &EntityIndex;
}

pub struct RawEntity<T> {
    value: T,
    index: EntityIndex,
}

impl<T> GetRawEntityDefinition<T> for RawEntity<T> {
    fn value_ref(&self) -> &T {
        &self.value
    }

    fn index_ref(&self) -> &EntityIndex {
        &self.index
    }
}
