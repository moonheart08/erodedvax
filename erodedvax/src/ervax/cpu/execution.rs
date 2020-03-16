use num_traits::{FromPrimitive, ToPrimitive};

use crate::ervax::cpu::{
    bus::VAXBus,
    mmu::VAXMMU,
    PrivilegeMode,
};

/// An ExecutionContext is the enviornment within which the emulated system executes, and it handles
/// all major aspects of the emulated system. Used to create, start, and stop the emulated CPU, it's
/// memory, and it's attached IO devices.
pub struct ExecutionContext {
    halted: bool,

    /// System Control Block Base.
    scbb: u32, 

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

/// Getters and setters for the Processor Status Longword
impl ExecutionContext {
    #[inline]
    fn get_psl_bit(&self, bit: u8) -> bool {
        assert!(bit < 32);
        let mask = 0x01 << bit;
        
        ((self.psl & mask) >> bit) != 0
    }

    #[inline]
    fn set_psl_bit(&mut self, bit: u8, val: bool) {
        assert!(bit < 32);
        let mask = 0x01 << bit;
        self.psl &= !mask;
        self.psl |= (val as u32) << bit;
    }

    #[inline]
    pub fn get_cur_priv_mode(&self) -> PrivilegeMode {
        PrivilegeMode::from_u32((self.psl & 0x0300_0000) >> 24).unwrap()
    }

    #[inline]
    pub fn set_cur_priv_mode(&mut self, m: PrivilegeMode) {
        let x = (m.to_u32().unwrap() << 24) & 0x0300_0000;
        self.psl &= !0x0300_0000;
        self.psl |= x;
    }
    #[inline]
    pub fn get_trace_pending(&self) -> bool {
        self.get_psl_bit(30)
    }
    #[inline]
    pub fn set_trace_pending(&mut self, val: bool) {
        self.set_psl_bit(30, val)
    }
    #[inline]
    pub fn get_first_part_done(&self) -> bool {
        self.get_psl_bit(27)
    }
    #[inline]
    pub fn set_first_part_done(&mut self, val: bool) {
        self.set_psl_bit(27, val)
    }
    #[inline]
    pub fn get_interrupt_stack(&self) -> bool {
        self.get_psl_bit(26)
    }
    #[inline]
    pub fn set_interrupt_stack(&mut self, val: bool) {
        self.set_psl_bit(26, val)
    }
    #[inline]
    pub fn get_decimal_overflow_enable(&self) -> bool {
        self.get_psl_bit(7)
    }
    #[inline]
    pub fn set_decimal_overflow_enable(&mut self, val: bool) {
        self.set_psl_bit(7, val)
    }
    #[inline]
    pub fn get_floating_underflow_enable(&self) -> bool {
        self.get_psl_bit(6)
    }
    #[inline]
    pub fn set_floating_underflow_enable(&mut self, val: bool) {
        self.set_psl_bit(6, val)
    }
    #[inline]
    pub fn get_integer_overflow_enable(&self) -> bool {
        self.get_psl_bit(5)
    }
    #[inline]
    pub fn set_integer_overflow_enable(&mut self, val: bool) {
        self.set_psl_bit(5, val)
    }
    #[inline]
    pub fn get_trace_enable(&self) -> bool {
        self.get_psl_bit(4)
    }
    #[inline]
    pub fn set_trace_enable(&mut self, val: bool) {
        self.set_psl_bit(4, val)
    }
    #[inline]
    pub fn get_negative(&self) -> bool {
        self.get_psl_bit(3)
    }
    #[inline]
    pub fn set_negative(&mut self, val: bool) {
        self.set_psl_bit(3, val)
    }
    #[inline]
    pub fn get_zero(&self) -> bool {
        self.get_psl_bit(1)
    }
    #[inline]
    pub fn set_zero(&mut self, val: bool) {
        self.set_psl_bit(1, val)
    }
    #[inline]
    pub fn get_carry(&self) -> bool {
        self.get_psl_bit(0)
    }
    #[inline]
    pub fn set_carry(&mut self, val: bool) {
        self.set_psl_bit(0, val)
    }
}

/// Execution.
impl ExecutionContext {
    /// Execute one step. Does not necessarily map to a single cycle.
    /// Usually takes however many cycles it needs to execute the next instruction.
    pub fn execute_step(&mut self) -> bool {
        unimplemented!()
    }
}

/// Creating an ExecutionContext
impl ExecutionContext {
    pub fn new() -> ExecutionContext {
        ExecutionContext {
            halted: true,
            scbb: 0,
            pcbb: 0,
            ksp: 0,
            esp: 0,
            ssp: 0,
            usp: 0,
            isp: 0,
            psl: 0,
            pc: 0,
            gpr: [0; 14],
            bus: VAXBus::new(524288, vec![]),
            mmu: VAXMMU::new(),
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn psl_get_set() {
        let mut exec = ExecutionContext::new();
        assert_eq!(exec.get_cur_priv_mode(), PrivilegeMode::Kernel);
        exec.set_cur_priv_mode(PrivilegeMode::User);
        assert_eq!(exec.get_cur_priv_mode(), PrivilegeMode::User);
    }
}