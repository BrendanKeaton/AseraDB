use crate::core::Operand;

#[derive(Default, Debug)]
pub struct ConditionsObject {
    objectOne: String,
    objectTwo: String,
    objectOneIsField: bool, // is object one a literal or field
    objecttwoIsField: bool, // is object two a literal or field
    operand: Operand,
}
