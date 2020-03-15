pub mod regfile;
pub mod instruction;
pub mod mmu;
pub mod sysclk;
pub mod bus;

/// Range of 0 through 15, 4 bit.
pub struct RegID(u8);

pub enum PrivilegeMode {
    Kernel = 0,
    Executive = 1,
    Supervisor = 2,
    User = 3,
}