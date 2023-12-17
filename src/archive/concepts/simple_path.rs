use crate::{error::KnowledgeGraphError, graph::KnowledgeGraph};

// Implement on AdjacencyMatrix
pub trait SimplePath {
    fn is_valid_simple_path(&self) -> Result<bool, KnowledgeGraphError>;
    fn try_simple_path_validity(&self) -> Result<(), KnowledgeGraphError>;
}

pub trait IndexDefinedSimplePath {}

// pub struct KeyDefinedPath {
//     edge_type: EdgeTypeKeyRef,
//     start: E
// }

// pub struct IndexDefinedPath {

// }
