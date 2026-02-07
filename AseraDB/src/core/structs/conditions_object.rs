use crate::core::Operand;

#[derive(Default, Debug)]
pub struct ConditionsObject {
    pub object_one: String,
    pub object_two: String,
    pub object_one_is_field: bool, // is object one a literal or field
    pub object_two_is_field: bool, // is object two a literal or field
    pub operand: Operand,
}
