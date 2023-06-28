use stacked_linear_algebra_graph::graph::graph::Graph as SparseLinearAlgebraGraph;

use crate::error::KnowledgeGraphError;

pub struct KnowledgeGraph {
    sparse_linear_algebra_graph: SparseLinearAlgebraGraph,
}

impl KnowledgeGraph {
    pub fn new_default() -> Result<Self, KnowledgeGraphError> {
        Ok(Self {
            sparse_linear_algebra_graph: SparseLinearAlgebraGraph::with_initial_capacity(
                &256, &256, &256,
            )?,
        })
    }
}

pub(crate) trait AsSparseLinearAlgebraGraph {
    fn sparse_linear_algebra_graph_ref(&self) -> &SparseLinearAlgebraGraph;
    fn sparse_linear_algebra_graph_mut_ref(&mut self) -> &mut SparseLinearAlgebraGraph;
}

impl AsSparseLinearAlgebraGraph for KnowledgeGraph {
    fn sparse_linear_algebra_graph_ref(&self) -> &SparseLinearAlgebraGraph {
        &self.sparse_linear_algebra_graph
    }

    fn sparse_linear_algebra_graph_mut_ref(&mut self) -> &mut SparseLinearAlgebraGraph {
        &mut self.sparse_linear_algebra_graph
    }
}
