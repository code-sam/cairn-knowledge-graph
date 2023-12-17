use crate::graph::ValueType;

pub trait EntityValue: ValueType {}

pub trait SetEntityValue<T: EntityValue> {}

pub trait GetEntityValue<T: EntityValue> {}
