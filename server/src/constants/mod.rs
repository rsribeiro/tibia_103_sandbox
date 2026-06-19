use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum ObjectUpdateType {
    Remove = 0,
    Add = 1,
    Update = 2,
}