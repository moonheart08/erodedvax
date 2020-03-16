use crate::ervax::cpu::{
    bus::VAXBus,
    mmu::VAXMMU,
};

/// An ExecutionContext is the enviornment within which the emulated system executes, and it handles
/// all major aspects of the emulated system. Used to create, start, and stop the emulated CPU, it's
/// memory, and it's attached IO devices.
pub struct ExecutionContext {
    halted: bool,

    /// System Control Block Base.
    scbb: u32, 
    /// Interrupt Priority Level.
    ipl: u8, 

    /// Proess Control Block Base.
    pcbb: u32,
    
    /// Kernel Stack Pointer.
    ksp: u32,
    /// Executive Stack Pointer.
    esp: u32,
    /// Supervisor Stack Pointer.
    ssp: u32,
    /// User Stack Pointer.
    usp: u32,
    /// Interrupt Stack Pointer.
    isp: u32,

    /// Processor Status Longword.
    psl: u32,

    /// Program counter.
    pc: u32,

    /// System's general purpose registers. 14 of 16. SP and PC are stored seperately.
    gpr: [u32;14],

    /// System main bus, also contains all attached IO devices.
    bus: VAXBus,

    /// System MMU. When enabled, memory reads/writes are passed through it first.
    mmu: VAXMMU,
}


impl ExecutionContext {
    /// Execute one step. Does not necessarily map to a single cycle.
    /// Usually takes however many cycles it needs to execute the next instruction.
    pub fn execute_step(&mut self) -> bool {
        unimplemented!()
    }
}