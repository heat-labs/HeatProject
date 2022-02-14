#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HType {
    Bool,
    U8,
    U16,
    U32,
    U64,
}

/// size of `hType::Bool` in bytes
pub const BOOL_SIZE: usize = 1;

/// size of `hType::U8` in bytes
pub const U8_SIZE: usize = 1;

/// size of `hType::U16` in bytes
pub const U16_SIZE: usize = 2;

/// size of `hType::U32` in bytes
pub const U32_SIZE: usize = 4;

/// size of `hType::U64` in bytes
pub const U64_SIZE: usize = 8;


pub fn get_size(h_type: HType) -> usize {
    return match h_type {
        HType::Bool => BOOL_SIZE,
        HType::U8 => U8_SIZE,
        HType::U16 => U16_SIZE,
        HType::U32 => U32_SIZE,
        HType::U64 => U64_SIZE,
    }
}