use crate::instruction::Instruction;
use crate::types;
use uuid::Uuid;
use lib_heat_spec::h_type::HType;
use crate::types::VirtualObject;

pub type FrameAddress = Uuid;

/// frames are independent virtual machines that
/// * have their own stack
/// * have no dependency to any other frame other than the frame's children
pub struct Frame {
    pub address: FrameAddress,
    pub instructions: Vec<Instruction>,
    pub local: Vec<types::VirtualObject>,
    pub stack: Vec<types::VirtualObject>,
    pub operand_stack: Vec<types::VirtualObject>,
    pub pc: u64,
}

impl Default for Frame {
    fn default() -> Self {
        Frame{
            address: Uuid::new_v4(),
            instructions: Default::default(),
            local: Default::default(),
            stack: Default::default(),
            operand_stack: Default::default(),
            pc: 0
        }
    }
}

impl Frame {

    pub fn new(uuid: Uuid) -> Frame {
        Frame {
            address: uuid,
            instructions: Default::default(),
            local: Default::default(),
            stack: Default::default(),
            operand_stack: Default::default(),
            pc: 0
        }
    }

    /// Allocate the `HType` and `push_back` it to the frame's stack
    ///
    /// ## Examples
    /// ```
    /// use libvirt::frame::Frame;
    ///
    /// // use the default frame
    /// let mut frame: Frame = Default::default();
    ///
    /// // allocate a bool inside the stack
    /// frame.AllocateInStack(HType::Bool);
    /// assert_eq!(frame.stack.len(), 1);
    ///
    /// let obj = frame.stack.pop()?;
    /// assert_eq!(obj.data_type, HType::Bool);
    /// assert_eq!(obj.data.capacity(), 1);
    /// ```
    pub fn allocate_in_stack(&mut self, htype: HType) {
        self
            .stack
            .push(VirtualObject::new_empty(htype));
    }

    pub fn get_front_in_stack(&self, offset: usize) -> Option<&VirtualObject> {
        self.stack.get(self.stack.len() - 1 - offset)
    }

    pub fn get_mut_front_in_stack(&mut self, offset: usize) -> Option<&mut VirtualObject> {
        let len = self.stack.len()-1;
        self.stack.get_mut(len-offset)
    }

    pub fn get_front_in_op_stack(&self, offset: usize) -> Option<&VirtualObject> {
        let len = self.stack.len()-1;
        self.stack.get(len-offset)
    }

    pub fn get_mut_front_in_op_stack(&mut self, offset: usize) -> Option<&mut VirtualObject> {
        self.operand_stack.get_mut(self.stack.len() - 1 - offset)
    }

    /// Clear instruction storage and resets the program counter
    pub fn clear_instructions(&mut self) {
        self.pc = 0;
        self.instructions.clear();
    }
}