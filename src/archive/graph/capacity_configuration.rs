pub struct CapacityConfiguration {
    entity_capacity: usize,
    entity_type_capacity: usize,
    relationship_type_capacity: usize,
}

impl CapacityConfiguration {
    pub fn new(
        entity_capacity: usize,
        entity_type_capacity: usize,
        relationship_type_capacity: usize,
    ) -> Self {
        Self {
            entity_capacity,
            entity_type_capacity,
            relationship_type_capacity,
        }
    }

    pub fn entity_capacity_ref(&self) -> &usize {
        &self.entity_capacity
    }

    pub fn entity_type_capacity_ref(&self) -> &usize {
        &self.entity_type_capacity
    }

    pub fn relationship_type_capacity_ref(&self) -> &usize {
        &self.relationship_type_capacity
    }
}
