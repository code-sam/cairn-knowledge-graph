use std::marker::PhantomData;
use std::sync::Arc;

use graphblas_sparse_linear_algebra::collections::sparse_matrix::{
    MatrixElement, SetMatrixElement, Size, SparseMatrix, SparseMatrixTrait,
};
use graphblas_sparse_linear_algebra::collections::sparse_vector::{
    SparseVector, SparseVectorTrait,
};
use graphblas_sparse_linear_algebra::context::Context;

use crate::error::GraphComputingError;
use crate::graph::index::ElementCount;
use crate::graph::index::Index;
use crate::graph::value_type::NativeDataType as GraphNativeDataType;
use crate::graph::value_type::ValueType;
use crate::graph::value_type::{
    implement_macro_for_all_native_value_types, ConvertScalarToMatrixType,
};
use crate::graph::vertex::{Vertex, VertexKeyRef, VertexTrait};

use crate::graph::indexer::{Indexer as VertexIndexer, IndexerTrait};

pub type SparseVertexMatrix<T: ValueType> = SparseMatrix<T>;
pub type SparseVertexVector<T: ValueType> = SparseVector<T>;

#[derive(Clone, Debug)]
pub(crate) struct VertexStore<T: ValueType> {
    vertices: SparseVertexVector<T>,
    indexer: VertexIndexer,
}

// pub(crate) trait VertexStoreTrait<T: ValueType> {
//     fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;

//     fn vertices_ref(&self) -> &SparseMatrix<T>;
//     fn vertices_mut_ref(&mut self) -> &mut SparseMatrix<T>;
// }

impl<T: ValueType> VertexStore<T> {
    pub(crate) fn with_initial_capacity(
        context: &Arc<Context>,
        inital_vertex_capacity: &ElementCount,
    ) -> Result<Self, GraphComputingError> {
        Ok(Self {
            vertices: SparseVertexVector::new(context, inital_vertex_capacity)?,
            indexer: VertexIndexer::with_initial_capacity(context, inital_vertex_capacity)?,
        })
    }
}

pub(super) trait VertexStoreTrait<T: ValueType> {
    fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;

    fn indexer_ref(&self) -> &VertexIndexer;
    fn indexer_mut_ref(&mut self) -> &mut VertexIndexer;

    fn vertex_vector_ref(&self) -> &SparseVertexVector<T>;
    fn vertex_vector_mut_ref(&mut self) -> &mut SparseVertexVector<T>;
}

impl<T: ValueType> VertexStoreTrait<T> for VertexStore<T> {
    fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
        self.vertices.resize(*new_capacity)?;
        Ok(())
    }

    fn indexer_ref(&self) -> &VertexIndexer {
        &self.indexer
    }
    fn indexer_mut_ref(&mut self) -> &mut VertexIndexer {
        &mut self.indexer
    }

    fn vertex_vector_ref(&self) -> &SparseVertexVector<T> {
        &self.vertices
    }
    fn vertex_vector_mut_ref(&mut self) -> &mut SparseVertexVector<T> {
        &mut self.vertices
    }
}

// impl<T: MatrixDataType> SetVertexData<T> for VertexStore<T> {
//     fn set_vertex_value(&mut self, index: Index, value: T) -> Result<(), GraphComputingError> {
//         self.vertices.set_element((index, index, value).into())?;
//         Ok(())
//     }
// }

// macro_rules! set_sparse_matrix_capacity {
//     ($vertices_typed:ident) => {
//         self.$vertices_typed.resize(&target_size)?;
//     };
// }

// impl SetCapacity for VertexStore {
//     fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//         let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//         implement_macro_with_typed_graph_indentifier_for_all_matrix_data_types!(set_sparse_matrix_capacity, vertices);
//         Ok(())
//     }
// }

// macro_rules! implement_set_capacity {
//     ($dummy:literal, $($y:ident),+) => {
//         impl SetCapacity for VertexStore {
//             fn set_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//                 let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//                 implement_set_capacity!($($y),+);
//     };
//     ($type_id:ident, $($y:ident),*) => {
//         paste::paste! {
//             self.[<vertices $type_id>].resize(&target_size)?;
//         }
//         implement_set_capacity!($($y),*);
//     };
//     ($type_id:ident) => {
//                 paste::paste! {
//                     self.[<vertices $type_id>].resize(&target_size)?;
//                 }
//                 Ok(())
//             }
//         }
//     }
// }

// macro_rules! implement_macro_for_all_graph_data_typed_parameter {
//     ($macro_identifier:ident) => {
//         $macro_identifier!(
//             0,
//             _bool,
//             _i8,
//             _i16,
//             _i32,
//             _i64,
//             _u8,
//             _u16,
//             _u32,
//             _u64,
//             _isize,
//             _usize,
//             _char,
//             _unit
//         );
//     };
// }

// implement_macro_for_all_graph_data_typed_parameter!(implement_set_capacity);

// pub(crate) trait SetCapacityTyped<G: GraphNativeDataType, M: MatrixDataType> {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;
// }

// macro_rules! implement_set_capacity_typed {
//     ($vertices_typed:ident, $graph_data_type:ty, $matrix_data_type:ty) => {
//         impl SetCapacityTyped<$graph_data_type, $matrix_data_type> for VertexStore {
//             fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//                 let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//                 Ok(self.$vertices_typed.resize(&target_size)?)
//             }
//         }
//     };
// }

// impl SetCapacityTyped<bool> for VertexStore {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//         let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//         Ok(self.vertices_bool.resize(&target_size)?)
//     }
// }

// implement_macro_with_typed_graph_indentifier_for_all_graph_and_matrix_data_types!(implement_set_capacity_typed, vertices);

// pub(crate) trait VertexData {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError>;
// }

// macro_rules! resize_vertices {
//     ($vertices_typed:ident) => {
//         self.$vertices_typed.resize(&target_size);
//     };
// }

// impl VertexData for VertexStore {
//     fn expand_capacity(&mut self, new_capacity: &ElementCount) -> Result<(), GraphComputingError> {
//         let target_size = Size::new(new_capacity.clone(), new_capacity.clone());
//         implement_macro_with_typed_indentifier_for_all_native_data_types!(resize_vertices, vertices);
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    use graphblas_sparse_linear_algebra::{
        collections::sparse_vector::SparseVector,
        context::{Context as GraphblasContext, Mode as GraphblasMode},
        index::ElementIndex as GraphblasElementIndex,
    };

    // #[test]
    // fn add_new_vertex() {
    //     let graphblas_context = GraphblasContext::init_ready(GraphblasMode::NonBlocking).unwrap();

    //     let mut store = VertexStore::<u8>::with_initial_capacity(&graphblas_context, &10).unwrap();

    //     let vertex_1 = Vertex::new(String::from("key"), 1u8);

    //     let index_1 = store.add_new_vertex(vertex_1.clone()).unwrap();
    //     assert!(store.is_valid_index(&index_1).unwrap());
    //     assert!(store.is_valid_key(vertex_1.key_ref()));
    // }
}