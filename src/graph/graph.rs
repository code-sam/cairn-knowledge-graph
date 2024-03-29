use std::sync::Arc;

use graphblas_sparse_linear_algebra::{
    context::{Context as GraphblasContext, Mode as GraphblasMode},
    util::ElementIndex as GraphblasElementIndex,
    value_types::sparse_vector::SparseVector,
};
use hashbrown::HashMap;

use super::edge::{EdgeType, EdgeTypeIndex, EdgeTypeRef};
use super::vertex::{Vertex, VertexIndex, VertexKey};
use crate::graph::edge::adjacency_matrix::AdjacencyMatrix;

use crate::error::{GraphComputingError, LogicError, LogicErrorType, UserError, UserErrorType};
use crate::graph::indexed_data_store::data_store::IndexedDataStore;
use crate::operations::{add_edge_type::AddEdgeType, drop_edge_type::DropEdgeType};

// NOTE: by default, SuiteSparse:GraphBLAS uses Compressed Sparse Row (CSR) format.
// Row operations should therefore be faster.
// TODO: review performance optimizations by using row operations, instead of column operations.

// pub type VertexIndex = IndexedDataStoreIndex;

// pub type VertexIndex = ElementIndex;
// pub type EdgeTypeIndex = IndexedDataStoreIndex;

pub type ElementCount = ElementIndex;
pub(crate) type ElementIndex = GraphblasElementIndex;

// TODO: should the scope of this trait be expanded to include most of the methods implemented below?
// For now, the design philosofy has been to only define user-exposed behaviour as a trait.
pub trait GraphTrait {
    fn number_of_vertices(&self) -> Result<ElementCount, GraphComputingError>;
    fn number_of_edge_types(&self) -> Result<ElementCount, GraphComputingError>;
    fn vertex_capacity(&self) -> Result<ElementIndex, GraphComputingError>;
    // TODO: number of edges
    // TODO: number of edges per edge type, etc
}

// pub struct Graph<VertexKey: Hash + Eq + PartialEq, EdgeType: Hash + Eq + PartialEq> {
#[derive(Clone, Debug)]
pub struct Graph {
    graphblas_context: Arc<GraphblasContext>,

    vertex_store: IndexedDataStore<Vertex>,
    vertex_key_to_vertex_index_map: HashMap<VertexKey, VertexIndex>, // maps a vertex key to a Vertex
    // vertex_set: FxHashSet<String>,
    // edge_types: IndexedDataStore<EdgeType>,
    adjacency_matrices: IndexedDataStore<AdjacencyMatrix>,
    // edges: IndexedDataStore<Vec<DirectedEdge>>, // first dimension over edge_type, second over adjacency_matrix element index
    edge_type_to_edge_type_index_map: HashMap<EdgeType, EdgeTypeIndex>, // maps an edge type key to an adjacency matrix
                                                                        // edge_set: FxHashSet<String>,                // TODO: type, unique connections
}

// let mut map: FxHashMap<String, ElementIndex> = FxHashMap::default();

impl GraphTrait for Graph {
    fn number_of_vertices(&self) -> Result<ElementCount, GraphComputingError> {
        let number_of_vertices = self
            .index_mask_with_all_vertices()
            .number_of_stored_elements()?;
        Ok(number_of_vertices)
    }

    fn number_of_edge_types(&self) -> Result<ElementCount, GraphComputingError> {
        let number_of_edge_types = self
            .index_mask_with_all_adjacency_matrices()
            .number_of_stored_elements()?;
        Ok(number_of_edge_types)
    }

    fn vertex_capacity(&self) -> Result<ElementIndex, GraphComputingError> {
        Ok(self.vertex_store.get_capacity()?)
    }

    // TODO: number of edges
    // TODO: number of edges for edge type
}

// impl Graph<VertexKey, EdgeKey> {
impl<'g> Graph {
    pub fn new(
        initial_vertex_capacity: ElementCount,
        initial_edge_type_capacity: ElementCount,
    ) -> Result<Self, GraphComputingError> {
        let graphblas_context = GraphblasContext::init_ready(GraphblasMode::NonBlocking)?;

        let mut vertex_key_to_vertex_index_map: HashMap<VertexKey, VertexIndex> =
            HashMap::default();
        vertex_key_to_vertex_index_map.reserve(initial_vertex_capacity);

        let mut edge_type_to_edge_type_index_map: HashMap<EdgeType, EdgeTypeIndex> =
            HashMap::default();
        edge_type_to_edge_type_index_map.reserve(initial_edge_type_capacity);

        // let mut edge_set: FxHashSet<EdgeKey> = FxHashSet::default();
        // edge_set.reserve(initial_edge_capacity);

        let mut graph: Graph = Self {
            graphblas_context: graphblas_context.clone(),

            vertex_store: IndexedDataStore::with_capacity(
                initial_vertex_capacity,
                graphblas_context.clone(),
            )?,
            vertex_key_to_vertex_index_map,

            // edge_types: IndexedDataStore::with_capacity(&edge_capacity),
            adjacency_matrices: IndexedDataStore::with_capacity(
                initial_edge_type_capacity,
                graphblas_context.clone(),
            )?,
            // edges: IndexedDataStore::with_capacity(&initial_edge_capacity), // TODO: consider if this can be made more efficient by reserving less memory
            edge_type_to_edge_type_index_map,
            // edge_set,
        };

        // allocate a dummy adjacency matrix to support self.expand_adjacency_matrices_to_match_target_capacity(),
        // TODO: research a more elegant alternative
        let dummy_edge_type = EdgeType::from("Dummy_at_init");
        graph.add_new_edge_type(dummy_edge_type.clone())?;
        graph.drop_edge_type_with_key(dummy_edge_type.as_str())?;

        Ok(graph)
    }

    pub(crate) fn graphblas_context_ref(&self) -> &Arc<GraphblasContext> {
        &self.graphblas_context
    }
    // pub(crate) fn graphblas_context_mut_ref(&mut self) -> &mut Arc<GraphblasContext> {
    //     &mut self.graphblas_context.clone()
    // }

    pub(crate) fn vertex_store_ref(&self) -> &IndexedDataStore<Vertex> {
        &self.vertex_store
    }
    pub(crate) fn vertex_store_mut_ref(&mut self) -> &mut IndexedDataStore<Vertex> {
        &mut self.vertex_store
    }

    pub(crate) fn vertex_key_to_vertex_index_map_ref(&self) -> &HashMap<VertexKey, VertexIndex> {
        &self.vertex_key_to_vertex_index_map
    }
    pub(crate) fn vertex_key_to_vertex_index_map_mut_ref(
        &mut self,
    ) -> &mut HashMap<VertexKey, VertexIndex> {
        &mut self.vertex_key_to_vertex_index_map
    }

    pub(crate) fn adjacency_matrices_ref(&self) -> &IndexedDataStore<AdjacencyMatrix> {
        &self.adjacency_matrices
    }
    pub(crate) fn adjacency_matrices_mut_ref(&mut self) -> &mut IndexedDataStore<AdjacencyMatrix> {
        &mut self.adjacency_matrices
    }

    pub(crate) fn edge_type_to_edge_type_index_map_ref(&self) -> &HashMap<EdgeType, EdgeTypeIndex> {
        &self.edge_type_to_edge_type_index_map
    }
    pub(crate) fn edge_type_to_edge_type_index_map_mut_ref(
        &mut self,
    ) -> &mut HashMap<EdgeType, EdgeTypeIndex> {
        &mut self.edge_type_to_edge_type_index_map
    }

    pub(crate) fn expand_adjacency_matrices_to_match_vertex_capacity(
        &mut self,
    ) -> Result<(), GraphComputingError> {
        // REVIEW: would it be more efficient to allocate a freed adjacency matrix at matrix initialization, instead of doing this check everytime?
        // if self.adjacency_matrices.get_number_of_stored_and_reusable_elements()? > 0 {
        match self.adjacency_matrices.get_ref(EdgeTypeIndex::new(0)) {
            // this line required the allocation of a dummy adjacency matrix at graph initialization. Review if a more elegant solution can be used.
            Err(_) => Ok(()), // TODO: check error type, pass error if not index-out-bounds
            Ok(adjacency_matrix) => {
                let target_capacity = self.vertex_capacity()?;
                if target_capacity > adjacency_matrix.get_vertex_capacity()? {
                    let resize_adjacency_matrix = |adjacency_matrix: &mut AdjacencyMatrix| -> Result<(), GraphComputingError> {
                            adjacency_matrix.resize(target_capacity) // REVIEW: return error instead of panic
                        };
                    self.adjacency_matrices.map_mut_all(resize_adjacency_matrix)
                } else {
                    Ok(())
                }
            }
        }
        // } else {
        //     Ok(())
        // }
    }

    pub(crate) fn get_edge_adjacency_matrix_ref(
        &self,
        edge_type: &EdgeTypeRef,
    ) -> Result<&AdjacencyMatrix, GraphComputingError> {
        match self.edge_type_to_edge_type_index_map.get(edge_type) {
            None => Err(UserError::new(
                UserErrorType::EdgeTypeDoesNotExist,
                format!("Edge type {} does not exist", edge_type),
                None,
            )
            .into()),
            Some(&index) => match self.adjacency_matrices.get_ref(index) {
                Ok(adjacency_matrix) => Ok(adjacency_matrix),
                Err(_) => Err(LogicError::new(
                    // TODO: match actual error type
                    LogicErrorType::Other,
                    format!(
                        "No adjacency matrix at mapped edge type index [{}]",
                        index.index()
                    ),
                    None,
                )
                .into()),
            },
        }
    }

    pub(crate) fn get_edge_adjacency_matrix_mut_ref(
        &mut self,
        edge_type: &EdgeTypeRef,
    ) -> Result<&mut AdjacencyMatrix, GraphComputingError> {
        match self.edge_type_to_edge_type_index_map.get(edge_type) {
            None => Err(UserError::new(
                UserErrorType::EdgeTypeDoesNotExist,
                format!("Edge type {} does not exist", edge_type),
                None,
            )
            .into()),
            Some(&index) => match self.adjacency_matrices.get_mut_ref(index) {
                Ok(adjacency_matrix) => Ok(adjacency_matrix),
                Err(_) => Err(LogicError::new(
                    // TODO: match actual error type
                    LogicErrorType::Other,
                    format!(
                        "No adjacency matrix at mapped edge type index [{}]",
                        index.index()
                    ),
                    None,
                )
                .into()),
            },
        }
    }

    // fn get_adjacency_matrix_target_capacity(&self) -> Result<VertexIndex, GraphComputingError> {
    //     Ok(self.vertex_values.get_capacity()?)
    // }

    pub(crate) fn index_mask_with_all_vertices(&self) -> &SparseVector<bool> {
        self.vertex_store.mask_with_valid_indices_ref()
    }

    pub(crate) fn index_mask_with_all_adjacency_matrices(&self) -> &SparseVector<bool> {
        self.adjacency_matrices.mask_with_valid_indices_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::operations::add_vertex::AddVertex;
    use crate::operations::read_vertex_value::ReadVertexValue;

    #[test]
    fn new_graph() {
        let graph = Graph::new(10, 20).unwrap();
        assert_eq!(graph.number_of_vertices().unwrap(), 0);
        // assert_eq!(graph.number_of_edge_types().unwrap(), 0); // TODO: fix this
    }

    #[test]
    fn graph_isolation() {
        let mut graph_1 = Graph::new(10, 20).unwrap();
        let mut graph_2 = Graph::new(10, 20).unwrap();

        let vertex_key = String::from("A key");
        let vertex_property_1 = String::from("Graph 1");
        let vertex_property_2 = String::from("Graph 2");

        let vertex_to_add_1 = Vertex::new(vertex_key.clone(), vertex_property_1.clone().into());
        graph_1
            .add_or_replace_vertex(vertex_to_add_1.clone())
            .unwrap();

        let vertex_to_add_2 = Vertex::new(vertex_key.clone(), vertex_property_2.clone().into());
        graph_2
            .add_or_replace_vertex(vertex_to_add_2.clone())
            .unwrap();

        assert_eq!(
            *graph_1.vertex_value(&vertex_key).unwrap(),
            vertex_to_add_1.value()
        );

        assert_eq!(
            *graph_2.vertex_value(&vertex_key).unwrap(),
            vertex_to_add_2.value()
        );
    }

    // TODO: Test vertex capacity
    // TODO: test number of stored vertices
    // TODO: test number of stored edge types
}
