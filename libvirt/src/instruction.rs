use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Clone, Debug)]
pub struct Instruction {
    pub opcode: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64
}

impl Instruction {
    pub fn from(instruction: &[u8]) -> Instruction {
        assert_eq!(instruction.len(), lib_heat_spec::instruction::SIZE as usize);
        let mut rdr = Cursor::new(instruction);
        let opcode = rdr.read_u64::<BigEndian>().unwrap();
        let arg1 = rdr.read_u64::<BigEndian>().unwrap();
        let arg2 = rdr.read_u64::<BigEndian>().unwrap();
        let arg3 = rdr.read_u64::<BigEndian>().unwrap();

        return Instruction{
            opcode,
            arg1,
            arg2,
            arg3
        }
    }
}