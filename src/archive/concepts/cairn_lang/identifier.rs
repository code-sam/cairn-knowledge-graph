use super::{
    namespace::{GetNamespaceElements, NamespaceHierarchy},
    namespace_separator::CAIRN_LANG_NAMESPACE_SEPARATOR,
};

pub struct RegisteredIdentifier {}

// pub trait CreateEntityKey {
//     fn from_namespace(namespace: &impl GetNamespaceElements, local: &str) -> EntityKey {
//         // TODO: introduce struct for local, or otherwise self-document namespace-local
//         // https://en.wikipedia.org/wiki/Namespace
//         let number_of_namespace_elements = namespace.elements_ref().len();
//         let mut identifier = String::with_capacity(number_of_namespace_elements * 2 + 1);
//         for element_index in 0..number_of_namespace_elements {
//             identifier.push_str(namespace.elements_ref()[element_index].as_str());
//             identifier.push_str(CAIRN_LANG_NAMESPACE_SEPARATOR);
//         }
//         identifier.push_str(local);
//         return identifier;
//     }
// }

// impl CreateEntityKey for EntityKey {}
