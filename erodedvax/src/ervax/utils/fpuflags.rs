use bitfield::bitfield;
use core::arch::x86_64;

bitfield! {
    pub struct FPUState(u32);
    impl Debug;

    pub invalid_op, set_invalid_op: 0;
    pub divzero, set_divzero: 1;
    pub overflow, set_overflow: 2;
    pub underflow, set_underflow: 3;
}

pub const SAVED_FLAGS_MASK: u32   = 0b000000_1_111111_111_1;
pub const CLEARED_FLAGS_MASK: u32 = 0b111111_0_000000_111_0;
pub const SET_FLAGS_MASK: u32     = 0b000000_1_111111_000_1;
pub trait VAXFloatOps<T> {
    fn vax_add(self, other: T) -> (T, FPUState);
    fn vax_sub(self, other: T) -> (T, FPUState);
    fn vax_mul(self, other: T) -> (T, FPUState);
    fn vax_div(self, other: T) -> (T, FPUState);
    fn vax_mod(self, other: T) -> (T, FPUState);
}

// Welcome to hell.
//TODO: figure out how to test this. 

#[inline(always)]
fn save_fp_state() -> u32 {
    unsafe { x86_64::_mm_getcsr() }
}

#[inline(always)]
fn setup_vax_state() {
    unsafe {
        x86_64::_MM_SET_ROUNDING_MODE(x86_64::_MM_ROUND_TOWARD_ZERO);
        x86_64::_MM_SET_FLUSH_ZERO_MODE(x86_64::_MM_FLUSH_ZERO_ON); 
    } 
}

#[inline(always)]
fn restore_fp_state(s: u32) {
    unsafe { x86_64::_mm_setcsr(s) }
}

fn store_vax_state() -> FPUState {
    unsafe {
    let s = x86_64::_MM_GET_EXCEPTION_STATE();
    let mut v = FPUState(0);

    v.set_invalid_op((s & x86_64::_MM_EXCEPT_INVALID) != 0);
    v.set_divzero((s & x86_64::_MM_EXCEPT_DIV_ZERO) != 0);
    v.set_overflow((s & x86_64::_MM_EXCEPT_OVERFLOW) != 0);
    v.set_underflow((s & x86_64::_MM_EXCEPT_UNDERFLOW) != 0);
    v
    }
}

impl VAXFloatOps<f32> for f32 {
    fn vax_add(self, other: f32) -> (f32, FPUState) {
        let s = save_fp_state();
        setup_vax_state();
        let v = self + other;
        let flags = store_vax_state();
        restore_fp_state(s);
        return (v, flags);
    }

    fn vax_sub(self, other: f32) -> (f32, FPUState) {
        let s = save_fp_state();
        setup_vax_state();
        let v = self - other;
        let flags = store_vax_state();
        restore_fp_state(s);
        return (v, flags);
    }

    fn vax_mul(self, other: f32) -> (f32, FPUState) {
        let s = save_fp_state();
        setup_vax_state();
        let v = self * other;
        let flags = store_vax_state();
        restore_fp_state(s);
        return (v, flags);
    }

    fn vax_div(self, other: f32) -> (f32, FPUState) {
        let s = save_fp_state();
        setup_vax_state();
        let v = self / other;
        let flags = store_vax_state();
        restore_fp_state(s);
        return (v, flags);
    }

    fn vax_mod(self, other: f32) -> (f32, FPUState) {
        let s = save_fp_state();
        setup_vax_state();
        let v = self % other;
        let flags = store_vax_state();
        restore_fp_state(s);
        return (v, flags);
    }
}