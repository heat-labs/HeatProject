use byteorder::{ByteOrder};
use lib_heat_spec::opcode;
pub struct Instruction {
    pub opcode: String,
    pub arg1: String,
    pub arg2: String,
    pub arg3: String,
}

impl Instruction {
    pub fn from(instruction_str: String) -> Result<Instruction, String> {
        let mut parts: Vec<&str> = instruction_str.split(" ").collect();
        if parts.len() > 4 {
            return Result::Err("invalid number of parts in instruction".to_string());
        }

        for _ in 0..4-parts.len() {
            parts.push("0");
        }

        return Result::Ok(Instruction{
            opcode: parts[0].to_string(),
            arg1: parts[1].to_string(),
            arg2: parts[2].to_string(),
            arg3: parts[3].to_string()
        });
    }

    pub fn to_byte_code(self) -> Result<Vec<u8>, String> {
        let opcode = string_to_opcode(self.opcode);
        let arg1: u64 = match self.arg1.parse::<u64>() {
            Ok(arg) => arg,
            Err(err) => return Err(format!("Invalid argument 1: {}", err))
        };
        let arg2: u64 = match self.arg2.parse::<u64>() {
            Ok(arg) => arg,
            Err(err) => return Err(format!("Invalid argument 2: {}", err))
        };
        let arg3: u64 = match self.arg3.parse::<u64>() {
            Ok(arg) => arg,
            Err(err) => return Err(format!("Invalid argument 3: {}", err))
        };

        let src = vec!(opcode, arg1, arg2, arg3);
        let mut dest = [0u8;32];
        byteorder::BigEndian::write_u64_into(&src, &mut dest);
        return Ok(dest.to_vec());
    }
}

fn string_to_opcode(opcode: String) -> u64 {
    return match opcode.as_str() {
        "NONE" => opcode::NONE,
        "NEW_BOOL" => opcode::NEW_BOOL,
        "NEW_U8"   => opcode::NEW_U8,
        "NEW_U16"  => opcode::NEW_U16,
        "NEW_U32"  => opcode::NEW_U32,
        "NEW_U64"  => opcode::NEW_U64,
        "EQUAL" => opcode::EQUAL,
        "NOT" => opcode::NOT,
        "AND" => opcode::AND,
        "OR" => opcode::OR,
        _ => opcode::ILLEGAL
    }
}