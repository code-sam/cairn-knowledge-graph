use graphblas_sparse_linear_algebra::collections::sparse_matrix::MatrixElement;
use graphblas_sparse_linear_algebra::collections::sparse_matrix::SetMatrixElement;

use crate::error::GraphComputingError;
use crate::graph::edge::EdgeCoordinateDefinedByIndices;
use crate::graph::edge::EdgeCoordinateDefinedByIndicesTrait;
use crate::graph::edge::EdgeDefinedByIndices;
use crate::graph::edge::EdgeDefinedByIndicesTrait;
use crate::graph::edge::EdgeDefinedByKeys;
use crate::graph::edge_store::weighted_adjacency_matrix::WeightedAdjacencyMatrixTrait;
use crate::graph::edge_store::EdgeStore;
use crate::graph::edge_store::WeightedAdjacencyMatrix;
use crate::graph::value_type::{implement_macro_for_all_native_value_types, ValueType};

pub(crate) trait AddEdge<T: ValueType> {
    fn add_edge_defined_by_indices(
        &mut self,
        edge: &EdgeDefinedByIndices<T>,
    ) -> Result<(), GraphComputingError>;

    fn add_edge_defined_by_indices_unchecked(
        &mut self,
        edge: &EdgeDefinedByIndices<T>,
    ) -> Result<(), GraphComputingError>;
}

// macro_rules! implement_add_edge {
//     ($value_type:ty) => {
//         impl AddEdge<$value_type> for EdgeStore<$value_type> {

//         }
//     };
// }

// implement_macro_for_all_native_value_types!(implement_add_edge);
