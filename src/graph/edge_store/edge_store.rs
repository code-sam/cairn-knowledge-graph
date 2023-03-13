use std::sync::Arc;

use graphblas_sparse_linear_algebra::error::SparseLinearAlgebraError;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use super::adjacency_matrix;
use super::weighted_adjacency_matrix::WeightedAdjacencyMatrix;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::{Size, SparseMatrixTrait};
use graphblas_sparse_linear_algebra::context::Context as GraphblasContext;

use crate::error::{GraphComputingError, LogicError, LogicErrorType};
use crate::graph::edge::EdgeTypeKeyRef;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrixTrait;
use crate::graph::graph::EdgeTypeIndex;
use crate::graph::index::ElementCount;
use crate::graph::indexer::Indexer as EdgeTypeIndexer;
use crate::graph::indexer::IndexerTrait;
use crate::graph::value_type::ValueType;

#[derive(Clone, Debug)]
pub(crate) struct EdgeStore<T: ValueType> {
    graphblas_context: Arc<GraphblasContext>,
    adjacency_matrices: Vec<WeightedAdjacencyMatrix<T>>,
    edge_type_indexer: EdgeTypeIndexer,
}

impl<T: ValueType> EdgeStore<T> {
    pub(crate) fn with_initial_capacity(
        graphblas_context: &Arc<GraphblasContext>,
        initial_vertex_capacity: &ElementCount,
        initial_edge_type_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        Ok(Self {
            graphblas_context: graphblas_context.clone(),
            edge_type_indexer: EdgeTypeIndexer::with_initial_capacity(
                graphblas_context,
                initial_edge_type_capacity,
            )?,
            adjacency_matrices: Vec::<WeightedAdjacencyMatrix<T>>::with_capacity(
                initial_edge_type_capacity.clone(),
            ),
        })
    }
}

pub(crate) trait EdgeStoreTrait<T: ValueType> {
    fn adjacency_matrices_ref(&self) -> &[WeightedAdjacencyMatrix<T>];
    fn adjacency_matrices_mut_ref(&mut self) -> &mut [WeightedAdjacencyMatrix<T>];
    fn adjacency_matrices_mut(&mut self) -> &mut Vec<WeightedAdjacencyMatrix<T>>;

    fn try_adjacency_matrix_ref(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix<T>, GraphComputingError>;
    fn try_adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix<T>, GraphComputingError>;

    fn adjacency_matrix_ref_unchecked(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix<T>;
    fn adjacency_matrix_mut_ref_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &mut WeightedAdjacencyMatrix<T>;

    fn adjacency_matrix_ref_for_key(
        &self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<&WeightedAdjacencyMatrix<T>, GraphComputingError>;
    fn adjacency_matrix_mut_ref_for_key(
        &mut self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<&mut WeightedAdjacencyMatrix<T>, GraphComputingError>;

    fn edge_type_indexer_ref(&self) -> &EdgeTypeIndexer;
    fn edge_type_indexer_mut_ref(&mut self) -> &mut EdgeTypeIndexer;

    ///
    fn resize_adjacency_matrices(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError>;
}

impl<T: ValueType> EdgeStoreTrait<T> for EdgeStore<T> {
    fn adjacency_matrices_ref(&self) -> &[WeightedAdjacencyMatrix<T>] {
        self.adjacency_matrices.as_slice()
    }

    fn adjacency_matrices_mut_ref(&mut self) -> &mut [WeightedAdjacencyMatrix<T>] {
        self.adjacency_matrices.as_mut_slice()
    }

    fn adjacency_matrices_mut(&mut self) -> &mut Vec<WeightedAdjacencyMatrix<T>> {
        &mut self.adjacency_matrices
    }

    fn edge_type_indexer_ref(&self) -> &EdgeTypeIndexer {
        &self.edge_type_indexer
    }

    fn edge_type_indexer_mut_ref(&mut self) -> &mut EdgeTypeIndexer {
        &mut self.edge_type_indexer
    }

    fn adjacency_matrix_ref_unchecked(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &WeightedAdjacencyMatrix<T> {
        &self.adjacency_matrices[*edge_type_index]
    }

    fn adjacency_matrix_mut_ref_unchecked(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> &mut WeightedAdjacencyMatrix<T> {
        &mut self.adjacency_matrices[*edge_type_index]
    }

    fn adjacency_matrix_ref_for_key(
        &self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<&WeightedAdjacencyMatrix<T>, GraphComputingError> {
        Ok(&self.adjacency_matrices[*self.edge_type_indexer.try_index_for_key(edge_type_key)?])
    }

    fn adjacency_matrix_mut_ref_for_key(
        &mut self,
        edge_type_key: &EdgeTypeKeyRef,
    ) -> Result<&mut WeightedAdjacencyMatrix<T>, GraphComputingError> {
        Ok(&mut self.adjacency_matrices
            [*self.edge_type_indexer.try_index_for_key(edge_type_key)?])
    }

    fn try_adjacency_matrix_ref(
        &self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&WeightedAdjacencyMatrix<T>, GraphComputingError> {
        match self.adjacency_matrices.get(*edge_type_index) {
            Some(adjacency_matrix) => Ok(adjacency_matrix),
            None => Err(LogicError::new(
                LogicErrorType::EdgeTypeMustExist,
                format!("No edge type for edge type index: {}", edge_type_index),
                None,
            )
            .into()),
        }
    }

    fn try_adjacency_matrix_mut_ref(
        &mut self,
        edge_type_index: &EdgeTypeIndex,
    ) -> Result<&mut WeightedAdjacencyMatrix<T>, GraphComputingError> {
        match self.adjacency_matrices.get_mut(*edge_type_index) {
            Some(adjacency_matrix) => Ok(adjacency_matrix),
            None => Err(LogicError::new(
                LogicErrorType::EdgeTypeMustExist,
                format!("No edge type for edge type index: {}", edge_type_index),
                None,
            )
            .into()),
        }
    }

    fn resize_adjacency_matrices(
        &mut self,
        new_vertex_capacity: ElementCount,
    ) -> Result<(), GraphComputingError> {
        self.map_mut_all_adjacency_matrices(
            |adjacency_matrix: &mut WeightedAdjacencyMatrix<T>| {
                adjacency_matrix.resize(new_vertex_capacity)
                // .sparse_matrix_mut_ref()
                // .resize(&(new_vertex_capacity, new_vertex_capacity).into())
            },
        )?;
        Ok(())
    }
}

impl<T: ValueType> EdgeStore<T> {
    /// Apply function to all adjacency matrices
    pub(crate) fn map_mut_all_adjacency_matrices<F>(
        &mut self,
        function_to_apply: F,
    ) -> Result<(), GraphComputingError>
    where
        F: Fn(&mut WeightedAdjacencyMatrix<T>) -> Result<(), GraphComputingError> + Send + Sync,
    {
        self.adjacency_matrices
            .as_mut_slice()
            .into_par_iter()
            .try_for_each(function_to_apply)?;
        Ok(())
    }
}
