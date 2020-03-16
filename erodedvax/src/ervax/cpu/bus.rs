// System prefers to operate on longwords. 
// Instruction fetching performs a longword fetch.
//
// Longword and word reads/writes are not always aligned.
// Cross-page read/writes may be performed if both pages are
// on the same device and the device addresses are contiguous.

pub trait VAXBusDevice {

}

// Implementation detail. Should be replaced eventually, as it's not as efficient as i'd like and
// significantly restricts memory layout.
struct VAXBusPage {
    upper: u32, // Upper half of the address fed to the target device.
    device: u16, // The device, referenced by ID.
}

// 0x0000_0000 through 0x3FFF_FFFF is considered RAM space.
pub struct VAXBus {
    devices: Vec<Box<dyn VAXBusDevice>>,
    ram: Vec<u8>,
    bus_map: Vec<VAXBusPage>, // VERY inefficient.
}

