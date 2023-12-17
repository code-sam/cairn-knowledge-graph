use super::namespace_separator::{NamespaceSeparatorRef, CAIRN_LANG_NAMESPACE_SEPARATOR};

pub type NamespaceElement = String;
pub type NamespaceElementRef = str;

// TODO: think of a better name
pub const NAMESPACE_LOCAL: &str = "namespace";
pub const NAMESPACE_NAMESPACE: &str = "namespace";

pub trait NamespaceBuilder {
    fn push_element_unchecked(&mut self, element: &NamespaceElementRef);
    fn from_elements_unchecked(elements: &[&str]) -> NamespaceHierarchy;
    // TODO: check namespace element validity (doesn't contain namespace separator)
}

pub trait GetNamespaceElements {
    fn elements_ref(&self) -> &[NamespaceElement];
}

#[derive(Debug, Clone)]
pub struct NamespaceHierarchy {
    namespace_hierarchy: Vec<NamespaceElement>,
}

pub struct NamespaceModel {}

pub struct ModelledNamespace {}

pub struct NamespaceInstance {}

pub struct NamespaceConcept {}

pub struct NamespaceConceptConfiguration {}

// pub struct ModelledNamespace

impl NamespaceBuilder for NamespaceHierarchy {
    fn push_element_unchecked(&mut self, element: &NamespaceElementRef) {
        self.namespace_hierarchy.push(element.to_owned());
    }

    fn from_elements_unchecked(elements: &[&NamespaceElementRef]) -> NamespaceHierarchy {
        let mut namespace_elements: Vec<String> = Vec::with_capacity(elements.len());
        for element in elements.into_iter() {
            namespace_elements.push(element.to_string())
        }
        return NamespaceHierarchy {
            namespace_hierarchy: namespace_elements,
        };
    }
}

impl GetNamespaceElements for NamespaceHierarchy {
    fn elements_ref(&self) -> &[NamespaceElement] {
        &self.namespace_hierarchy.as_slice()
    }
}

// pub fn create_entity_key_from_namespace(
//     namespace_elements: &impl GetNamespaceElements,
//     namespace_separator: &NamespaceSeparatorRef,
//     local: &str,
// ) -> EntityKey {
//     // TODO: introduce struct for local, or otherwise self-document namespace-local
//     // https://en.wikipedia.org/wiki/Namespace
//     let number_of_namespace_elements = namespace_elements.elements_ref().len();
//     let mut identifier = String::with_capacity(number_of_namespace_elements * 2 + 1);
//     for element_index in 0..number_of_namespace_elements {
//         identifier.push_str(namespace_elements.elements_ref()[element_index].as_str());
//         identifier.push_str(namespace_separator);
//     }
//     identifier.push_str(local);
//     return identifier;
// }
