use stacked_linear_algebra_graph::graph::graph::{VertexIndex, VertexTypeIndex};
use stacked_linear_algebra_graph::operators::add::AddVertexType;

use crate::{
    error::KnowledgeGraphError,
    graph::{AsStackedLinearAlgebraGraph, KnowledgeGraph},
};

pub trait EnableValueTypeVertexFilter {
    fn enable(&mut self) -> Result<ValueTypeVertexFilterConfiguration, KnowledgeGraphError>;
}

// impl EnableValueTypeVertexFilter for KnowledgeGraph {
//     fn enable(&mut self) -> Result<ValueTypeVertexFilterConfiguration, KnowledgeGraphError> {
//         let vertex_type_index = self
//             .stacked_linear_algebra_graph_mut_ref()
//             .add_new_vertex_type("cairn_lang::value_type")?;

//         let vertex_type_bool_index = self
//             .stacked_linear_algebra_graph_mut_ref()
//             .add_new_vertex_type("cairn_lang::value_type::bool")?;

//         todo!()
//     }
// }

pub trait ValueTypeVertexFilterConfigurationData {}

pub struct ValueTypeVertexFilterConfiguration {
    entityVertexFilterIndex: VertexTypeIndex,

    conceptVertexIndex: VertexIndex,
}
