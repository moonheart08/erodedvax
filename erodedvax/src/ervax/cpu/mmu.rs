use num_derive::*;
use num_traits::{FromPrimitive, ToPrimitive};


use crate::ervax::{
    cpu::{
        PrivilegeMode
    },
    utils::addr_lw_trim,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum PTEProtectionCode {
                             // KESU
    NoAccess      = 0b0000,  // ----
    ZeroPage      = 0b0001,  // RRRR
    KernW         = 0b0010,  // W---
    KernR         = 0b0011,  // R---
    UserW         = 0b0100,  // WWWW
    ExecW         = 0b0101,  // WW--
    ExecRKernW    = 0b0110,  // WR--
    ExecR         = 0b0111,  // RR--
    SuperW        = 0b1000,  // WWW-
    SuperRExecW   = 0b1001,  // WWR-
    SuperRKernW   = 0b1010,  // WRR-
    SuperR        = 0b1011,  // RRR-
    UserRSuperW   = 0b1100,  // WWWR
    UserRExecW    = 0b1101,  // WWRR
    UserRKernW    = 0b1110,  // WRRR
    UserR         = 0b1111,  // RRRR
}

impl PTEProtectionCode {
    pub fn from_int(n: u8) -> Option<Self> {
        if n<=0b1111 {
            unsafe{
                Some(std::mem::transmute::<u8,Self>(n))
            }
        }else{
            None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MemoryAccessType {
    Read,
    Write,
}

impl PTEProtectionCode {
    pub fn can_access(self, mode: PrivilegeMode, access: MemoryAccessType) -> bool {
        let m = self.to_u8().unwrap();
        let rm = ((m & 0b1100) >> 2) as u8;
        let wm = !(m & 0b0011) as u8;


        match mode.to_u8().unwrap() {
            16..=u8::MAX => unreachable!(),

            0 => false,
            4 => true,

            v if v < wm => {
                true
            },

            v if (access == MemoryAccessType::Read) && (v <= rm)  => {
                true
            },

            _ => false,

        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PFN(u32);

impl From<u32> for PFN {
    fn from(v: u32) -> Self {
        PFN(v & 0x3FFF_F800 >> 11)
    }
}


pub struct VAXMMU {
    /// P0BR
    p0_base: u32,
    /// P0LR
    p0_len: u32,

    /// P1BR
    p1_base: u32,
    /// P1LR
    p1_len: u32,

    /// SBR
    sys_base: u32,
    /// SLR
    sys_len: u32,
}

/// Initialization
impl VAXMMU {
    pub fn new() -> Self {
        VAXMMU {
            p0_base: 0,
            p0_len: 0,
            p1_base: 0,
            p1_len: 0,
            sys_base: 0,
            sys_len: 0,
        }
    }
}

/// Setters/getters for region controls
impl VAXMMU {
    #[inline]
    pub fn set_p0_len(&mut self, len: u32) {
        debug_assert!(len < 8_388_609);
        self.p0_len = len;
    }
    #[inline]
    pub fn get_p0_len(&mut self) -> u32 {
        self.p0_len * 512
    }

    #[inline]
    pub fn set_p1_len(&mut self, len: u32) {
        debug_assert!(len < 8_388_609);
        self.p1_len = len;
    }
    #[inline]
    pub fn get_p1_len(&mut self) -> u32 {
        self.p1_len * 512
    }

    #[inline]
    pub fn set_sys_len(&mut self, len: u32) {
        debug_assert!(len < 8_388_609);
        self.sys_len = len;
    }
    #[inline]
    pub fn get_sys_len(&mut self) -> u32 {
        self.sys_len * 512
    }

    #[inline]
    pub fn set_p0_base(&mut self, base: u32) {
        self.p0_base = addr_lw_trim(base);
    }

    #[inline]
    pub fn set_p1_base(&mut self, base: u32) {
        self.p1_base = addr_lw_trim(base);
    }
}

/// Address translation
impl VAXMMU {
    /// Checks an address for validity
    /// See page 209 of the VAX Architecture Reference Manual (1987)
    pub fn is_address_valid(&self, addr: u32) -> bool {
        let region = VAXMMU::address_region(addr); 
        let maddr = addr & 0x3FFF_F800;
        
        match region {
            0 if maddr < (self.p0_len * 512) => true,
            1 if maddr > (self.p1_len * 512) => true,
            2 if maddr > (self.sys_len * 512) => true,
            _ => false,
        }
    }

    /// Returns 0, 1, 2, or 3, representing the region of the address.
    #[inline]
    pub fn address_region(addr: u32) -> u8 {
        ((addr & 0xC000_0000) >> 30) as u8
    }

    /// Gets the physical address of the PTE used to translate the input.
    /// Returns None if address is invalid or MMU is disabled
    pub fn get_pte_address(&self, translatee: u32) -> Option<u32> {
        if !self.is_address_valid(translatee) { return None; }

        let pfn: PFN = translatee.into();

        match VAXMMU::address_region(translatee) {
            0 if (pfn.0 > self.p0_len) =>
                Some((pfn.0 * 4) + self.p0_base),
            1 if (pfn.0 > self.p1_len) =>
                Some((pfn.0 * 4) + self.p1_base),
            2 if (pfn.0 > self.sys_len) =>
                Some((pfn.0 * 4) + self.sys_base),
            _ => None,
        }
    }
}

mod tests {
    use crate::ervax::cpu::mmu::*;
    #[test] 
    fn address_region_decode() {
        assert_eq!(VAXMMU::address_region(0x0000_0000), 0);
        assert_eq!(VAXMMU::address_region(0xC000_0000), 3);
    }
}