use crate::{error::KnowledgeGraphError, graph::KnowledgeGraph};

// Implement on AdjacencyMatrix
pub trait SimplePath {
    fn is_valid_simple_path(&self) -> Result<bool, KnowledgeGraphError>;
    fn try_simple_path_validity(&self) -> Result<(), KnowledgeGraphError>;
}

// pub trait KeyDefinedSimplePath {
//     fn is_simple_path(&self, edge_type: &EdgeTypeKeyRef) -> Result<bool, KnowledgeGraphError>;
//     fn try_simple_path_validity(
//         &self,
//         edge_type: &EdgeTypeKeyRef,
//     ) -> Result<(), KnowledgeGraphError>;
// }

// pub trait SimplePath {}

// pub struct KeyDefinedPath {
//     edge_type: EdgeTypeKeyRef,
//     start: E
// }

// pub struct IndexDefinedPath {

// }
