use crate::constraints::Constraints;
use crate::frame::Frame;
use lib_heat_spec;
use lib_heat_spec::h_type::HType;
use lib_heat_spec::opcode;
use crate::types::VirtualObject;

pub struct Interpreter {
    pub constraints: Constraints,
}

impl Interpreter {
    pub fn new(constraints: Constraints) -> Interpreter {
        return Interpreter { constraints };
    }

    /// Execute a frame within an interpreter
    pub fn execute_frame(&self, frame: &mut Frame) {
        loop {
            if frame.pc as usize == frame.instructions.len() { break; }
            let i = frame.instructions.get(frame.pc as usize).unwrap().clone();

            match i.opcode {
                opcode::NONE => {}
                opcode::NEW_BOOL => {
                    frame.allocate_in_stack(HType::Bool);
                }
                opcode::NEW_U8 => {
                    frame.allocate_in_stack(HType::U8);
                }
                opcode::NEW_U16 => {
                    frame.allocate_in_stack(HType::U16);
                }
                opcode::NEW_U32 => {
                    frame.allocate_in_stack(HType::U32);
                }
                opcode::NEW_U64 => {
                    frame.allocate_in_stack(HType::U64);
                }
                opcode::LOAD_BOOL => {
                    let val = frame.get_mut_front_in_stack(0).unwrap();
                    if val.data_type != HType::Bool {
                        panic!("trying to load into a non bool object");
                    }

                    val.set_bool(&(i.arg1 != 0));
                }
                opcode::LOAD_U8 => {
                    let val = frame.get_mut_front_in_stack(0).unwrap();
                    if val.data_type != HType::U8 {
                        panic!("trying to load into a non u8 object");
                    }

                    val.set_u8(&(i.arg1 as u8));
                }
                opcode::LOAD_U16 => {
                    let val = frame.get_mut_front_in_stack(0).unwrap();
                    if val.data_type != HType::U16 {
                        panic!("trying to load into a non u16 object");
                    }

                    val.set_u16(&(i.arg1 as u16));
                }
                opcode::LOAD_U32 => {
                    let val = frame.get_mut_front_in_stack(0).unwrap();
                    if val.data_type != HType::U32 {
                        panic!("trying to load into a non u32 object");
                    }

                    val.set_u32(&(i.arg1 as u32));
                }
                opcode::LOAD_U64 => {
                    let val = frame.get_mut_front_in_stack(0).unwrap();
                    if val.data_type != HType::U64 {
                        panic!("trying to load into a non u64 object");
                    }

                    val.set_u64(&(i.arg1 as u64));
                }
                opcode::STORE => {
                    let operand: VirtualObject = frame.get_front_in_op_stack(0).unwrap().clone();
                    frame.stack.push(operand);
                }
                opcode::ADD_U8 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U8);
                    res_obj.set_u8(&(val1.get_u8() + val2.get_u8()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::ADD_U16 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U16);
                    res_obj.set_u16(&(val1.get_u16() + val2.get_u16()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::ADD_U32 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U32);
                    res_obj.set_u32(&(val1.get_u32() + val2.get_u32()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::ADD_U64 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U64);
                    res_obj.set_u64(&(val1.get_u64() + val2.get_u64()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::SUB_U8 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U8);
                    res_obj.set_u8(&(val1.get_u8() - val2.get_u8()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::SUB_U16 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U16);
                    res_obj.set_u16(&(val1.get_u16() - val2.get_u16()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::SUB_U32 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U32);
                    res_obj.set_u32(&(val1.get_u32() - val2.get_u32()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::SUB_U64 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U64);
                    res_obj.set_u64(&(val1.get_u64() - val2.get_u64()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::DIV_U8 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U8);
                    res_obj.set_u8(&(val1.get_u8() / val2.get_u8()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::DIV_U16 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U16);
                    res_obj.set_u16(&(val1.get_u16() / val2.get_u16()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::DIV_U32 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U32);
                    res_obj.set_u32(&(val1.get_u32() / val2.get_u32()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::DIV_U64 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U64);
                    res_obj.set_u64(&(val1.get_u64() / val2.get_u64()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::MUL_U8 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U8);
                    res_obj.set_u8(&(val1.get_u8() * val2.get_u8()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::MUL_U16 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U16);
                    res_obj.set_u16(&(val1.get_u16() * val2.get_u16()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::MUL_U32 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U32);
                    res_obj.set_u32(&(val1.get_u32() * val2.get_u32()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::MUL_U64 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U64);
                    res_obj.set_u64(&(val1.get_u64() * val2.get_u64()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::PWR_U8 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U8);
                    res_obj.set_u8(&(val1.get_u8() ^ val2.get_u8()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::PWR_U16 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U16);
                    res_obj.set_u16(&(val1.get_u16() ^ val2.get_u16()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::PWR_U32 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U32);
                    res_obj.set_u32(&(val1.get_u32() ^ val2.get_u32()));
                    frame.operand_stack.push(res_obj);
                }
                opcode::PWR_U64 => {
                    let val1 = frame.get_front_in_stack(0).unwrap();
                    let val2 = frame.get_front_in_stack(1).unwrap();
                    let mut res_obj = VirtualObject::new_empty(HType::U64);
                    res_obj.set_u64(&(val1.get_u64() ^ val2.get_u64()));
                    frame.operand_stack.push(res_obj);
                }
                _ => {}
            }
            frame.pc += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use lib_heat_spec::h_type::{BOOL_SIZE, HType, U16_SIZE, U32_SIZE, U64_SIZE, U8_SIZE};
    use lib_heat_spec::opcode;
    use crate::constraints::Constraints;
    use crate::frame::Frame;
    use crate::instruction::Instruction;
    use crate::interpreter::Interpreter;
    use crate::types::VirtualObject;

    #[test]
    /// Performs stack allocation test on all HTypes using NEW_\[HType] instruction
    fn interpreter_frame_stack_allocation() {
        let i = Interpreter::new(Constraints::new_none());
        let mut frame = Frame::default();
        frame.instructions.push(Instruction {
            opcode: lib_heat_spec::opcode::NEW_BOOL,
            arg1: 0,
            arg2: 0,
            arg3: 0
        });
        frame.instructions.push(Instruction {
            opcode: lib_heat_spec::opcode::NEW_U8,
            arg1: 0,
            arg2: 0,
            arg3: 0
        });
        frame.instructions.push(Instruction {
            opcode: lib_heat_spec::opcode::NEW_U16,
            arg1: 0,
            arg2: 0,
            arg3: 0
        });
        frame.instructions.push(Instruction {
            opcode: lib_heat_spec::opcode::NEW_U32,
            arg1: 0,
            arg2: 0,
            arg3: 0
        });
        frame.instructions.push(Instruction {
            opcode: lib_heat_spec::opcode::NEW_U64,
            arg1: 0,
            arg2: 0,
            arg3: 0
        });
        i.execute_frame(&mut frame);

        assert_eq!(frame.stack.get(0).unwrap().data_type, HType::Bool);
        assert_eq!(frame.stack.get(0).unwrap().data.capacity(), BOOL_SIZE);

        assert_eq!(frame.stack.get(1).unwrap().data_type, HType::U8);
        assert_eq!(frame.stack.get(1).unwrap().data.capacity(), U8_SIZE);

        assert_eq!(frame.stack.get(2).unwrap().data_type, HType::U16);
        assert_eq!(frame.stack.get(2).unwrap().data.capacity(), U16_SIZE);

        assert_eq!(frame.stack.get(3).unwrap().data_type, HType::U32);
        assert_eq!(frame.stack.get(3).unwrap().data.capacity(), U32_SIZE);

        assert_eq!(frame.stack.get(4).unwrap().data_type, HType::U64);
        assert_eq!(frame.stack.get(4).unwrap().data.capacity(), U64_SIZE);
    }

    #[test]
    /// Performs LOAD_[HType] on `VirtualObjects` in stack
    fn interpreter_frame_stack_obj_load() {
        let i = Interpreter::new(Constraints::new_none());
        let mut frame = Frame::default();

        frame.stack.push(VirtualObject::new_empty(HType::Bool));
        frame.instructions.push(Instruction {
            opcode: lib_heat_spec::opcode::LOAD_BOOL,
            arg1: 1,
            arg2: 0,
            arg3: 0
        });
        i.execute_frame(&mut frame);


        frame.clear_instructions();
        frame.stack.push(VirtualObject::new_empty(HType::U8));
        frame.instructions.push(Instruction {
            opcode: lib_heat_spec::opcode::LOAD_U8,
            arg1: u8::MAX as u64,
            arg2: 0,
            arg3: 0
        });
        i.execute_frame(&mut frame);


        frame.clear_instructions();
        frame.stack.push(VirtualObject::new_empty(HType::U16));
        frame.instructions.push(Instruction {
            opcode: lib_heat_spec::opcode::LOAD_U16,
            arg1: u16::MAX as u64,
            arg2: 0,
            arg3: 0
        });
        i.execute_frame(&mut frame);


        frame.clear_instructions();
        frame.stack.push(VirtualObject::new_empty(HType::U32));
        frame.instructions.push(Instruction {
            opcode: lib_heat_spec::opcode::LOAD_U32,
            arg1: u32::MAX as u64,
            arg2: 0,
            arg3: 0
        });
        i.execute_frame(&mut frame);


        frame.clear_instructions();
        frame.stack.push(VirtualObject::new_empty(HType::U64));
        frame.instructions.push(Instruction {
            opcode: lib_heat_spec::opcode::LOAD_U64,
            arg1: u64::MAX,
            arg2: 0,
            arg3: 0
        });
        i.execute_frame(&mut frame);


        assert_eq!(frame.stack.get(0).unwrap().get_bool(), true);
        assert_eq!(frame.stack.get(1).unwrap().get_u8(), u8::MAX);
        assert_eq!(frame.stack.get(2).unwrap().get_u16(), u16::MAX);
        assert_eq!(frame.stack.get(3).unwrap().get_u32(), u32::MAX);
        assert_eq!(frame.stack.get(4).unwrap().get_u64(), u64::MAX);
    }

    #[test]
    /// Performs STORE on `VirtualObjects` in operand stack
    fn interpreter_frame_stack_operand_store() {
        let i = Interpreter::new(Constraints::new_none());

        let types = [HType::U8, HType::U16, HType::U32, HType::U64, HType::Bool];
        for h_type in types {
            let mut frame = Frame::default();
            frame.stack.push(VirtualObject::new_max(h_type));
            frame.instructions.push(Instruction {
                opcode: opcode::STORE,
                arg1: 0,
                arg2: 0,
                arg3: 0
            });
            i.execute_frame(&mut frame);
            assert_eq!(frame.stack.pop().unwrap().data_type, h_type, "checking both operand and the first stack item are the same type")
        }
    }


    #[test]
    /// Performs ADD_[HType] on `VirtualObjects` in operand stack
    fn interpreter_frame_arithmetic_add() {
        let interpreter = Interpreter::new(Constraints::new_none());

        let u8list:[[u8;3];4] = [
            [1, 2, 3],
            [5, 6, 11],
            [4, 1, 5],
            [7, 8, 15],
        ];

        let u16list:[[u16;3];4] = [
            [1, 2, 3],
            [5, 6, 11],
            [4, 1, 5],
            [7, 8, 15],
        ];

        let u32list:[[u32;3];4] = [
            [1, 2, 3],
            [5, 6, 11],
            [4, 1, 5],
            [7, 8, 15],
        ];

        let u64list:[[u64;3];4] = [
            [1, 2, 3],
            [5, 6, 11],
            [4, 1, 5],
            [7, 8, 15],
        ];


        for i in u8list {
            let mut frame = Frame::default();
            frame.stack.push(VirtualObject::from(i[0]));
            frame.stack.push(VirtualObject::from(i[1]));

            frame.instructions.push(Instruction {
                opcode: opcode::ADD_U8,
                arg1: 0, arg2: 0, arg3: 0
            });

            interpreter.execute_frame(&mut frame);
            assert_eq!(frame.operand_stack.pop().unwrap().get_u8(), i[2]);
        }

        for i in u16list {
            let mut frame = Frame::default();
            frame.stack.push(VirtualObject::from(i[0]));
            frame.stack.push(VirtualObject::from(i[1]));

            frame.instructions.push(Instruction {
                opcode: opcode::ADD_U16,
                arg1: 0, arg2: 0, arg3: 0
            });

            interpreter.execute_frame(&mut frame);
            assert_eq!(frame.operand_stack.pop().unwrap().get_u16(), i[2]);
        }

        for i in u32list {
            let mut frame = Frame::default();
            frame.stack.push(VirtualObject::from(i[0]));
            frame.stack.push(VirtualObject::from(i[1]));

            frame.instructions.push(Instruction {
                opcode: opcode::ADD_U32,
                arg1: 0, arg2: 0, arg3: 0
            });

            interpreter.execute_frame(&mut frame);
            assert_eq!(frame.operand_stack.pop().unwrap().get_u32(), i[2]);
        }

        for i in u64list {
            let mut frame = Frame::default();
            frame.stack.push(VirtualObject::from(i[0]));
            frame.stack.push(VirtualObject::from(i[1]));

            frame.instructions.push(Instruction {
                opcode: opcode::ADD_U64,
                arg1: 0, arg2: 0, arg3: 0
            });

            interpreter.execute_frame(&mut frame);
            assert_eq!(frame.operand_stack.pop().unwrap().get_u64(), i[2]);
        }
    }

    #[test]
    /// Performs SUB_[HType] on `VirtualObjects` in operand stack
    fn interpreter_frame_arithmetic_sub() {
        let interpreter = Interpreter::new(Constraints::new_none());

        let u8list:[[u8;3];4] = [
            [2, 5, 3],
            [2, 7, 5],
            [1, 12, 11],
            [5, 20, 15],
        ];

        let u16list:[[u16;3];4] = [
            [2, 5, 3],
            [2, 7, 5],
            [1, 12, 11],
            [5, 20, 15],
        ];

        let u32list:[[u32;3];4] = [
            [2, 5, 3],
            [2, 7, 5],
            [1, 12, 11],
            [5, 20, 15],
        ];

        let u64list:[[u64;3];4] = [
            [2, 5, 3],
            [2, 7, 5],
            [1, 12, 11],
            [5, 20, 15],
        ];


        for i in u8list {
            let mut frame = Frame::default();
            frame.stack.push(VirtualObject::from(i[0]));
            frame.stack.push(VirtualObject::from(i[1]));

            frame.instructions.push(Instruction {
                opcode: opcode::SUB_U8,
                arg1: 0, arg2: 0, arg3: 0
            });

            interpreter.execute_frame(&mut frame);
            assert_eq!(frame.operand_stack.pop().unwrap().get_u8(), i[2]);
        }

        for i in u16list {
            let mut frame = Frame::default();
            frame.stack.push(VirtualObject::from(i[0]));
            frame.stack.push(VirtualObject::from(i[1]));

            frame.instructions.push(Instruction {
                opcode: opcode::SUB_U16,
                arg1: 0, arg2: 0, arg3: 0
            });

            interpreter.execute_frame(&mut frame);
            assert_eq!(frame.operand_stack.pop().unwrap().get_u16(), i[2]);
        }

        for i in u32list {
            let mut frame = Frame::default();
            frame.stack.push(VirtualObject::from(i[0]));
            frame.stack.push(VirtualObject::from(i[1]));

            frame.instructions.push(Instruction {
                opcode: opcode::SUB_U32,
                arg1: 0, arg2: 0, arg3: 0
            });

            interpreter.execute_frame(&mut frame);
            assert_eq!(frame.operand_stack.pop().unwrap().get_u32(), i[2]);
        }

        for i in u64list {
            let mut frame = Frame::default();
            frame.stack.push(VirtualObject::from(i[0]));
            frame.stack.push(VirtualObject::from(i[1]));

            frame.instructions.push(Instruction {
                opcode: opcode::SUB_U64,
                arg1: 0, arg2: 0, arg3: 0
            });

            interpreter.execute_frame(&mut frame);
            assert_eq!(frame.operand_stack.pop().unwrap().get_u64(), i[2]);
        }
    }
}
