// TODO: implement primitives concepts

// TODO: support passing in a custom top-level namespace
// model the namespace as a Vec<String>, and validate validity (namespace seperator)

use stacked_linear_algebra_graph::{
    graph::graph::Graph as StackedLinearAlgebraGraph,
    operators::add::{AddEdgeType, AddVertex, AddVertexType},
};

use crate::{
    concepts::cairn_lang::{
        namespace::{
            GetNamespaceElements, NamespaceBuilder, NamespaceHierarchy, NAMESPACE_LOCAL,
            NAMESPACE_NAMESPACE,
        },
        namespace_separator::CAIRN_LANG_NAMESPACE_SEPARATOR,
    },
    error::KnowledgeGraphError,
    graph::{operations::AddRawEntityType, KnowledgeGraph},
};

use super::configuration::{
    GetKnowledgeGraphPrimitivesConfiguration, KnowledgeGraphPrimitivesConfiguration,
};

pub trait StoreCairnLangPrimitivesConfiguration {}

pub trait CairnLangPrimitiveConfiurationData {}

impl CairnLangPrimitiveConfiurationData for KnowledgeGraphPrimitivesConfiguration {}

pub trait ImplementPrimitiveConcepts {
    fn implement_primitive_concepts(
        &mut self,
        configuration: &impl GetKnowledgeGraphPrimitivesConfiguration,
    ) -> Result<KnowledgeGraphPrimitivesConfiguration, KnowledgeGraphError>;
}

impl ImplementPrimitiveConcepts for KnowledgeGraph {
    fn implement_primitive_concepts(
        &mut self,
        configuration: &impl GetKnowledgeGraphPrimitivesConfiguration,
    ) -> Result<KnowledgeGraphPrimitivesConfiguration, KnowledgeGraphError> {
        // let cairn_lang_namespace = Namespace::from_elements_unchecked(&[cairn_lang_namespace]);

        // // Cairn-lang namespace
        // let cairn_lang_namespace_vertex_type_key =
        //     graph_key_from_namespace(&cairn_lang_namespace, NAMESPACE_LOCAL);
        // let cairn_lang_vertex_type_index =
        //     self.add_new_raw_entity_type(cairn_lang_namespace_vertex_type_key.as_str())?;

        // // Namespace
        // let mut namespace_namespace = cairn_lang_namespace.clone();
        // namespace_namespace.push_element_unchecked(NAMESPACE_NAMESPACE);
        // let namespace_vertex_type_key =
        //     graph_key_from_namespace(&namespace_namespace, NAMESPACE_LOCAL);
        // let namespace_vertex_type_index = self.add_new_edge_type(&namespace_vertex_type_key)?;
        // let namespace_vertex_key = namespace_vertex_type_key.to_owned();
        // let namespace_vertex_index = self.add_new_vertex_defined_by_type_index_and_vertex_key(
        //     VertexDefinedByTypeIndexAndVertexKey::new(
        //         &namespace_vertex_type_index,
        //         namespace_vertex_key.as_str(),
        //         &true,
        //     ),
        // )?;

        // let cairn_lang_namespace_vertex_key = cairn_lang_namespace_vertex_type_key.to_owned();
        // let cairn_lang_vertex_type_index = self
        //     .add_new_vertex_defined_by_type_index_and_vertex_key(
        //         VertexDefinedByTypeIndexAndVertexKey::new(
        //             &namespace_vertex_type_index,
        //             cairn_lang_namespace_vertex_key.as_str(),
        //             &true,
        //         ),
        //     )?;

        // Vertex type
        // let mut vertex_type

        // sequence

        // TODO: add each vertex type as a vertex as well, add an isBool and isVertexType vertexType, and set the data type each vertex type in the respective vertex vector
        // TODO: use vertex type, not edge type
        // TODO
        // self.add_new_edge_type(
        //     format!(
        //         "{}{}{}",
        //         cairn_lang_namespace, NAMESPACE_SEPARATOR, "namespace"
        //     )
        //     .as_str(),
        // )?;
        // let cairn_lang_namespace_vertex_type_index =
        //     self.add_new_edge_type(cairn_lang_namespace)?;
        todo!()
    }
}
