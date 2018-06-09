extern crate libc;

#[repr(C)]
pub enum FieldTransformationType {
    None,
    Flip180Degrees
}