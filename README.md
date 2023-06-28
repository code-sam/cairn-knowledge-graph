![test](https://github.com/code-sam/cairn-knowledge-graph/actions/workflows/test_main_branch.yml/badge.svg?branch=main)
# Cairn Knowledge Graph
An embedded and in-memory knowledge graph for static specification analysis.

## Capabilities

### Graph type
The Cairn Knowledge Graph implements a [directed graph](https://en.wikipedia.org/wiki/Directed_graph). Each vertex holds a value. A vertex' value can be selected using a unique string, or an index value.

### Data types
A vertex can hold a value of the following types:
- unit, indicating the absence of a value
- string, containing [Unicode Scalar Values](https://www.unicode.org/glossary/#unicode_scalar_value),
- boolean,
- integer
- unsigned integer
- floating point, (IEEE 754-2008)

### ACID
Cairn Knowledge Graph does currently not guarantee [ACID](https://en.wikipedia.org/wiki/ACID) database transaction properties.

### Persistence
The graph resides in-memory and does currently not exist in persistent storage.

## Usage
Applications can include the cairn_knowledge_graph crate as a dependency. Building cairn_knowledge_graph requires that an ANSI C11 compatible C-compiler and CMake are installed.

## Minimum example
```rust
// use cairn_knowledge_graph::error::LinearAlgebraGraphComputingError;
// use cairn_knowledge_graph::graph::edge::DirectedEdgeDefinedByKeys;
// use cairn_knowledge_graph::graph::graph::Graph;
// use cairn_knowledge_graph::graph::vertex::{Vertex, VertexValue};
// use cairn_knowledge_graph::operations::add_edge::AddEdge;
// use cairn_knowledge_graph::operations::add_vertex::AddVertex;
// use cairn_knowledge_graph::operations::selection::operators::and::AndOperator;
// use cairn_knowledge_graph::operations::select_vertex::SelectVertex;
// use cairn_knowledge_graph::operations::selection::vertex_selection::VertexSelection;

// macro_rules! add_new_edge {
//   ($from_vertex:ident, $edge_type:ident, $to_vertex:ident, $graph:ident) => {
//      let edge = DirectedEdgeDefinedByKeys::new(
//          $from_vertex.clone().into(),
//          $edge_type.clone(),
//          $to_vertex.clone().into(),
//      );
//      $graph.add_edge_and_edge_type_using_keys(edge.clone()).unwrap();
//   };
// }
 
// fn main() {
//     let initial_vertex_capacity = 10;
//     let initial_edge_type_capacity = 10;
//     let mut graph = Graph::new(initial_vertex_capacity, initial_edge_type_capacity).unwrap();

//     let zero = Vertex::new(String::from("0"), 0u8.into());
//     graph.add_or_replace_vertex(zero.clone()).unwrap();

//     let one = Vertex::new(String::from("1"), 1u8.into());
//     graph.add_or_replace_vertex(one.clone()).unwrap();

//     let one_dot_one = Vertex::new(String::from("1.1"), 1.1f32.into());
//     graph.add_or_replace_vertex(one_dot_one.clone()).unwrap();

//     let negative_one = Vertex::new(String::from("-1"), (-1i8).into());
//     graph.add_or_replace_vertex(negative_one.clone()).unwrap();

//     let negative_one_dot_one = Vertex::new(String::from("-1.1"), (-1.1f32).into());
//     graph
//         .add_or_replace_vertex(negative_one_dot_one.clone())
//         .unwrap();

//     let integer = Vertex::new(String::from("integer"), String::from("integer").into());
//     graph.add_or_replace_vertex(integer.clone()).unwrap();

//     let real_number = Vertex::new(
//         String::from("real_number"),
//         String::from("real_number").into(),
//     );
//     graph.add_or_replace_vertex(real_number.clone()).unwrap();

//     let positive = Vertex::new(String::from("positive"), String::from("positive").into());
//     graph.add_or_replace_vertex(positive.clone()).unwrap();

//     let negative = Vertex::new(String::from("negative"), String::from("negative").into());
//     graph.add_or_replace_vertex(negative.clone()).unwrap();

//     let sign = String::from("sign");
//     let is_a = String::from("is_a");

//     add_new_edge!(negative_one_dot_one, is_a, real_number, graph);
//     add_new_edge!(negative_one_dot_one, sign, negative, graph);

//     add_new_edge!(negative_one, is_a, real_number, graph);
//     add_new_edge!(negative_one, is_a, integer, graph);
//     add_new_edge!(negative_one, sign, negative, graph);

//     add_new_edge!(zero, is_a, real_number, graph);
//     add_new_edge!(zero, is_a, integer, graph);

//     add_new_edge!(one, is_a, real_number, graph);
//     add_new_edge!(one, is_a, integer, graph);
//     add_new_edge!(one, sign, positive, graph);

//     add_new_edge!(one_dot_one, is_a, real_number, graph);
//     add_new_edge!(one_dot_one, sign, positive, graph);

//     let negative_selection = graph
//         .select_vertices_connected_to_vertex_by_key(sign, &"negative")
//         .unwrap();
//     let integer_selection = graph
//         .select_vertices_connected_to_vertex_by_key(is_a, &"integer")
//         .unwrap();

//     let negative_integer_selection = negative_selection.and(&integer_selection).unwrap();
//     let negative_integers = negative_integer_selection.vertex_values_ref().unwrap();

//     assert_eq!(negative_integers.len(), 1);
//     assert_eq!(negative_integers, vec!(&VertexValue::Integer8Bit(-1)));
//  }
 ```

## Architecture
Cairn Knowledge Graph is inspired by [LAGraph](https://github.com/GraphBLAS/LAGraph) and uses the same underlying GraphBLAS implementation from [Timothy A. Davis](https://github.com/DrTimothyAldenDavis/GraphBLAS).

## Licensing
cairn_knowledge_graph is licensed under [Creative Commons Attribution Non Commercial 4.0 International](https://creativecommons.org/licenses/by-nc/4.0/legalcode). For other licensing options, please contact Sam Dekker.

## Contributing
Awesome, contributions are welcome. cairn_knowledge_graph and your contribution may be relicensed and integrated into commercial software in the future. Therefore, you will be asked to agree to the [Contributor License Agreement](contributor-license-agreement.md) when you make a pull request.
