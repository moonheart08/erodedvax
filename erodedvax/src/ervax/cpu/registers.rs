#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum PrivRegisters {
    /// Kernel Stack Pointer
    KSP = 0,
    /// Executive Stack Pointer
    ESP = 1,
    /// Supervisor Stack Pointer
    SSP = 2,
    /// User Stack Pointer
    USP = 3,
    /// Interrupt Stack Pointer
    ISP = 4,
    /// Address Space Number
    ASN = 6, // Unsupported
    /// P0 Base Register
    P0BR = 8,
    /// P0 Length Register
    POLR = 9,
    /// P1 Base Register
    P1BR = 10,
    /// P1 Length Register
    P1LR = 11,
    /// System Base Register
    SBR = 12,
    /// System Length Register
    SLR = 13,
    /// CPU Identification
    CPUID = 14,
    /// Process Control Block Base
    PCBB = 16,
    /// System Control Block Base
    SCBB = 17,
    /// Interrupt Priority Level
    IPL = 18,
    /// AST Level
    ASTLVL = 19,
    /// Software Interrupt Request
    SIRR = 20,
    /// Software Interrupt Summary
    SISR = 21,
    /// Interval Clock Control
    ICCS = 24,
    /// Next Interval Count
    NICR = 25,
    /// Interval Count
    ICR = 26,
    /// Time of Year
    TODR = 27,
    /// Console Reciever Status
    RXCS = 32,
    /// Console Reciever Data Buffer
    RXDB = 33,
    /// Console Transmit Status
    TXCS = 34,
    /// Console Transmit Data buffer
    TXDB = 35,
    /// Memory Management Enable
    MAPEN = 56,
}