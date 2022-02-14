
pub const NONE: u64 = 0x00; // None

pub const NEW_BOOL: u64 = 0x01; // Allocates an object in the frame's stack of type bool
pub const NEW_U8: u64 = 0x02;   // Allocates an object in the frame's stack of type u8
pub const NEW_U16: u64 = 0x03;  // Allocates an object in the frame's stack of type u16
pub const NEW_U32: u64 = 0x04;  // Allocates an object in the frame's stack of type u32
pub const NEW_U64: u64 = 0x05;  // Allocates an object in the frame's stack of type u64


pub const EQUAL: u64 = 0x20; // Returns true if two values are equal
pub const NOT: u64 = 0x21;   // Returns not value of a boolean
pub const AND: u64 = 0x22;   // Returns not value of 2 booleans
pub const OR: u64 = 0x23;    // Returns or value of 2 booleans

pub const LOAD_BOOL: u64 = 0x30; // Load bool into stack
pub const LOAD_U8: u64 = 0x31;   // Load u8 into stack
pub const LOAD_U16: u64 = 0x32;  // Load u16 into stack
pub const LOAD_U32: u64 = 0x33;  // Load u32 into stack
pub const LOAD_U64: u64 = 0x34;  // Load u64 into stack

pub const STORE: u64 = 0x40; // Store from operand stack

pub const ADD_U8: u64 = 0x50;   // Pop 2 objects from stack and add them together u8
pub const ADD_U16: u64 = 0x51;  // Pop 2 objects from stack and add them together u16
pub const ADD_U32: u64 = 0x52;  // Pop 2 objects from stack and add them together u32
pub const ADD_U64: u64 = 0x53;  // Pop 2 objects from stack and add them together u64

pub const SUB_U8: u64 = 0x60;   // Pop 2 objects from stack and subtract them u8
pub const SUB_U16: u64 = 0x61;  // Pop 2 objects from stack and subtract them u16
pub const SUB_U32: u64 = 0x62;  // Pop 2 objects from stack and subtract them u32
pub const SUB_U64: u64 = 0x63;  // Pop 2 objects from stack and subtract them u64

pub const DIV_U8: u64 = 0x70;   // Pop 2 objects from stack and divide them u8
pub const DIV_U16: u64 = 0x71;  // Pop 2 objects from stack and divide them u16
pub const DIV_U32: u64 = 0x72;  // Pop 2 objects from stack and divide them u32
pub const DIV_U64: u64 = 0x73;  // Pop 2 objects from stack and divide them u64

pub const MUL_U8: u64 = 0x80;   // Pop 2 objects from stack and multiply them u8
pub const MUL_U16: u64 = 0x81;  // Pop 2 objects from stack and multiply them u16
pub const MUL_U32: u64 = 0x82;  // Pop 2 objects from stack and multiply them u32
pub const MUL_U64: u64 = 0x83;  // Pop 2 objects from stack and multiply them u64

pub const PWR_U8: u64 = 0x90;   // Pop 2 objects from stack and get the power of them u8
pub const PWR_U16: u64 = 0x91;  // Pop 2 objects from stack and get the power of them u16
pub const PWR_U32: u64 = 0x92;  // Pop 2 objects from stack and get the power of them u32
pub const PWR_U64: u64 = 0x93;  // Pop 2 objects from stack and get the power of them u64



pub const ILLEGAL: u64 = u64::MAX;    // ILLEGAL opcode
