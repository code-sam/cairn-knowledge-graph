use std::marker::PhantomData;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    MatrixElement, SetMatrixElement, Size, SparseMatrix, SparseMatrixTrait,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::SetVectorElement;
use graphblas_sparse_linear_algebra::context::Context;

use crate::error::GraphComputingError;
use crate::graph::index::ElementCount;
use crate::graph::index::Index;
use crate::graph::indexer::{Indexer, IndexerTrait};
use crate::graph::indexer::{NewIndex, NewIndexTrait};
use crate::graph::value_type::NativeDataType as GraphNativeDataType;
use crate::graph::value_type::ValueType;
use crate::graph::value_type::{
    implement_macro_for_all_native_value_types, ConvertScalarToMatrixType,
};
use crate::graph::vertex::{VertexDefinedByKey, VertexDefinedByKeyTrait, VertexKeyRef, VertexDefinedByIndex};
use crate::graph::vertex_store::vertex_store::{VertexStore, VertexStoreTrait};

use crate::graph::vertex_store::{VertexVector, VertexVectorTrait};

pub(crate) trait AddVertex<T: ValueType> {
    fn add_new_vertex(
        &mut self,
        vertex: VertexDefinedByIndex<T>,
    ) -> Result<NewIndex, GraphComputingError>;
    fn add_or_replace_vertex(
        &mut self,
        vertex: VertexDefinedByIndex<T>,
    ) -> Result<NewIndex, GraphComputingError>;
    fn add_or_update_vertex(
        &mut self,
        vertex: VertexDefinedByIndex<T>,
    ) -> Result<Option<NewIndex>, GraphComputingError>;
}

// TODO: review expansion of vertex matrix
macro_rules! implement_set_vertex_data {
    ($value_type:ty) => {
        impl AddVertex<$value_type> for VertexVector {
            fn add_new_vertex(
                &mut self,
                vertex: VertexDefinedByIndex<$value_type>,
            ) -> Result<NewIndex, GraphComputingError> {
                let index = self.indexer_mut_ref().add_new_key(vertex.key_ref())?; // TODO
                self.vertex_vector_mut_ref()
                    .set_element((index.index(), vertex.value_ref().clone()).into())?;
                Ok(index)
            }

            fn add_or_replace_vertex(
                &mut self,
                vertex: VertexDefinedByKey<$value_type>,
            ) -> Result<NewIndex, GraphComputingError> {
                let index = self
                    .indexer_mut_ref()
                    .add_or_replace_key(vertex.key_ref())?; // TODO
                self.vertex_vector_mut_ref()
                    .set_element((index.index(), vertex.value_ref().clone()).into())?;
                Ok(index)
            }

            fn add_or_update_vertex(
                &mut self,
                vertex: VertexDefinedByKey<$value_type>,
            ) -> Result<Option<NewIndex>, GraphComputingError> {
                match self.indexer_ref().index_for_key(vertex.key_ref()) {
                    Some(index_ref) => {
                        let index = index_ref.clone();
                        self.vertex_vector_mut_ref()
                            .set_element((index, vertex.value_ref().clone()).into())?;
                        Ok(None)
                    }
                    None => {
                        // REVIEW: can this arm be made faster with the knowledge that the vertex is new?
                        Ok(Some(self.add_new_vertex(vertex)?))
                    }
                }
            }
        }
    };
}

implement_macro_for_all_native_value_types!(implement_set_vertex_data);