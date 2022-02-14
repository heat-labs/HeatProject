pub struct Constraints {
    pub max_memory: u64,

    /// maximum number of allocation possible in the stack
    pub max_stack_allocation: u64,
}

impl Constraints {
    pub fn new(max_memory: u64, max_stack_allocation: u64) -> Constraints {
        return Constraints{ max_memory, max_stack_allocation }
    }
    pub fn new_none() -> Constraints {
        return Constraints{ max_memory: 0, max_stack_allocation: 0 };
    }
}
