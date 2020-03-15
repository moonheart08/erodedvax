pub mod fpuflags;

pub fn addr_lw_trim(inp: u32) -> u32 {
    inp & (!3)
}