use crate::graph::{EntityTypeIndex, ValueTypeIdentifier};

use super::RelationshipTypeIndex;

pub struct RawRelationshipType {
    index: RelationshipTypeIndex,
    value_type: ValueTypeIdentifier,
}

pub trait GetRawRelationshipTypeDefinition {
    fn index_ref(&self) -> &RelationshipTypeIndex;
}

impl GetRawRelationshipTypeDefinition for RawRelationshipType {
    fn index_ref(&self) -> &RelationshipTypeIndex {
        &self.index
    }
}
