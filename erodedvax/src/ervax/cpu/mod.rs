pub mod registers;
pub mod instrs;
pub mod mmu;
pub mod sysclk;
pub mod bus;
pub mod execution;
pub mod interrupts;

/// Range of 0 through 15, 4 bit.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RegID(u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub enum PrivilegeMode {
    Kernel = 0,
    Executive = 1,
    Supervisor = 2,
    User = 3,
}