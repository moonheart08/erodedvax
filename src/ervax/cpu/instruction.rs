use num_derive::*;
use num_traits::{FromPrimitive, ToPrimitive};


use crate::ervax::cpu::{RegID};

pub enum OperandWidth {
    Byte, // u8
    Word, // 16
    Longword, // u32
    Quadword, // u64
    Octaword, // u128
}

pub enum OperandMode {
    Literal(u8),
    Register(RegID),
    RegisterDeferred(RegID),
    AutoDecrement(RegID),
    AutoIncrement(RegID),
    AutoIncrementDeferred(RegID),
    ByteDisplacement(RegID, i8),
    ByteDisplacementDeferred(RegID, i8),
    WordDisplacement(RegID, i16),
    WordDisplacementDeferred(RegID, i16),
    LongwordDisplacement(RegID, i32),
    LongwordDisplacementDeferred(RegID, i32),
    Absolute(u32),
    Indexed(Box<OperandMode>), // TODO: find some way to pretty this up
    Immediate8(u8), // Needs to handle all possible value sizes, up to i128...
    Immediate16(u16),
    Immediate32(u32),
    Immediate64(u64),
    Immediate128(u128),
    BranchByte(u8),
    BranchWord(u8),
}

impl OperandMode {
    pub fn is_valid_indexed(&self) -> bool {
        use OperandMode::*;
        match self {
            Literal(_) | Indexed(_) | Register(_) |
            AutoDecrement(_) | AutoIncrement(_) | AutoIncrementDeferred(_) => false,
            _ => true,
        }
    }
}

pub enum OperandParseError {
    OutOfBytes,
    InvalidMode,
}

pub fn get_u16_from_stream<I>(bytes: &mut I) -> Option<u16>
    where I: Iterator<Item = u8>
{
    let mut e: [u8; 2] = [0; 2];
    let d: Vec<u8> = bytes.take(2)/*interactive*/.collect();
    if d.len() != 2 { return None }
    let c = &d[..e.len()];
    e.copy_from_slice(c); 
    Some(u16::from_le_bytes(e))
}

pub fn get_u32_from_stream<I>(bytes: &mut I) -> Option<u32>
    where I: Iterator<Item = u8>
{
    let mut e: [u8; 4] = [0; 4];
    let d: Vec<u8> = bytes.take(4).collect();
    if d.len() != 4 { return None }
    let c = &d[..e.len()];
    e.copy_from_slice(c); 
    Some(u32::from_le_bytes(e))
}

pub fn get_u64_from_stream<I>(bytes: &mut I) -> Option<u64>
    where I: Iterator<Item = u8>
{
    let mut e: [u8; 8] = [0; 8];
    let d: Vec<u8> = bytes.take(8).collect();
    if d.len() != 4 { return None }
    let c = &d[..e.len()];
    e.copy_from_slice(c); 
    Some(u64::from_le_bytes(e))
}

pub fn get_u128_from_stream<I>(bytes: &mut I) -> Option<u128>
    where I: Iterator<Item = u8>
{
    let mut e: [u8; 16] = [0; 16];
    let d: Vec<u8> = bytes.take(16).collect();
    if d.len() != 16 { return None }
    let c = &d[..e.len()];
    e.copy_from_slice(c); 
    Some(u128::from_le_bytes(e))
}

impl OperandMode {
    pub fn read_operand<I>(bytes: &mut I, width: OperandWidth) -> Result<OperandMode, OperandParseError>
        where I: Iterator<Item = u8>
    {
        
        if let Some(head) = bytes.next() {
            // Shortcut for checking for literals, as they have a unusual layout.
            if (head & 0xC0) >> 6 == 0 {
                return Ok(OperandMode::Literal(head & 0x3F));
            }

            let optype = (head & 0xF0) >> 4;
            let field = (head & 0x0F) >> 0;
            let reg = RegID(field);

            return match optype {
                0..=3 => unreachable!(), // Caught earlier. OperandMode::Literal
                4 => {
                    let mut bytes = bytes.peekable();
                    // Indexed mode, aka pain.
                    match bytes.peek() {
                        Some(v) if ((v & 0xF0) >>4) == 4 => {
                            // Someone tried to do OperandMode::Indexed powered recursion.
                            // This check ruins their hopes and dreams.
                            Err(OperandParseError::InvalidMode)
                        }
                        Some(_) => {
                            // And this is why the above check exists.
                            let submode = OperandMode::read_operand::<std::iter::Peekable<&mut I>>(&mut bytes, width);

                            match submode {
                                Err(v) => {
                                    Err(v)
                                },
                                Ok(op) if op.is_valid_indexed() => {
                                    Ok(OperandMode::Indexed(Box::new(op)))
                                }
                                _ => {
                                    Err(OperandParseError::InvalidMode)
                                }
                            }
                        }
                        None => {
                            Err(OperandParseError::OutOfBytes)
                        }
                    }
                }
                5 => Ok(OperandMode::Register(reg)),
                6 => Ok(OperandMode::RegisterDeferred(reg)),
                7 => Ok(OperandMode::AutoDecrement(reg)),
                8 if field != 0xF => Ok(OperandMode::AutoIncrement(reg)),


                8 if field == 0xF => {
                    match width {
                        // Very verbose, could be condensed.
                        // Maybe try and shrink down the
                        /*
                            match bytes.next() {
                                Some(v) => Ok(OperandMode::Immediate8(v)),
                                None => Err(OperandParseError::OutOfBytes),
                            }
                        */
                        // pieces
                        OperandWidth::Byte => {
                            match bytes.next() {
                                Some(v) => Ok(OperandMode::Immediate8(v)),
                                None => Err(OperandParseError::OutOfBytes),
                            }
                        }
                        OperandWidth::Word => {
                            match get_u16_from_stream(bytes) {
                                Some(v) => Ok(OperandMode::Immediate16(v)),
                                None => Err(OperandParseError::OutOfBytes),
                            }
                        }
                        OperandWidth::Longword => {
                            match get_u32_from_stream(bytes) {
                                Some(v) => Ok(OperandMode::Immediate32(v)),
                                None => Err(OperandParseError::OutOfBytes),
                            }
                        }
                        OperandWidth::Quadword => {
                            match get_u64_from_stream(bytes) {
                                Some(v) => Ok(OperandMode::Immediate64(v)),
                                None => Err(OperandParseError::OutOfBytes),
                            }
                        }
                        OperandWidth::Octaword => {
                            match get_u128_from_stream(bytes) {
                                Some(v) => Ok(OperandMode::Immediate128(v)),
                                None => Err(OperandParseError::OutOfBytes),
                            }
                        }
                    }
                }
                8 => unreachable!(),

                9 if field != 0xF => Ok(OperandMode::AutoIncrementDeferred(reg)),
                9 if field == 0xF => {
                    match get_u32_from_stream(bytes) {
                        Some(v) => Ok(OperandMode::Absolute(v)),
                        None => Err(OperandParseError::OutOfBytes),
                    }
                }
                9 => unreachable!(),

                10 => {
                    match bytes.next() {
                        Some(v) => Ok(OperandMode::ByteDisplacement(reg, v as i8)),
                        None => Err(OperandParseError::OutOfBytes),
                    }
                }

                11 => {
                    match get_u16_from_stream(bytes) {
                        Some(v) => Ok(OperandMode::WordDisplacement(reg, v as i16)),
                        None => Err(OperandParseError::OutOfBytes),
                    }
                }

                12 => {
                    match get_u32_from_stream(bytes) {
                        Some(v) => Ok(OperandMode::LongwordDisplacement(reg, v as i32)),
                        None => Err(OperandParseError::OutOfBytes),
                    }
                }

                13 => {
                    match bytes.next() {
                        Some(v) => Ok(OperandMode::ByteDisplacementDeferred(reg, v as i8)),
                        None => Err(OperandParseError::OutOfBytes),
                    }
                }

                14  => {
                    match get_u16_from_stream(bytes) {
                        Some(v) => Ok(OperandMode::WordDisplacementDeferred(reg, v as i16)),
                        None => Err(OperandParseError::OutOfBytes),
                    }
                }

                15  => {
                    match get_u32_from_stream(bytes) {
                        Some(v) => Ok(OperandMode::LongwordDisplacementDeferred(reg, v as i32)),
                        None => Err(OperandParseError::OutOfBytes),
                    }
                }

                16..=std::u8::MAX => unreachable!(),
            }

        } else {
            return Err(OperandParseError::OutOfBytes); // oh no, already out of bytes to nom? :<
        }
    }
}

pub enum FieldMode {
    Read,
    Modify,
    Write,
    Address,
    Bitfield,
    DataByte,
    DataWord,
    DataLong,
    VariableLengthTable, // CASE why.
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[repr(u16)]
pub enum InstructionType {
    ADAWI = 0x58,

    ADDB2 = 0x80,
    ADDB3 = 0x81,
    ADDW2 = 0xA0,
    ADDW3 = 0xA1,
    ADDL2 = 0xC0,
    ADDL3 = 0xC1,

    ADWC = 0xD8,

    ASHL = 0x78,
    ASHQ = 0x79,

    BICB2 = 0x8A,
    BICB3 = 0x8B,
    BICW2 = 0xAA,
    BICW3 = 0xAB,
    BICL2 = 0xCA,
    BICL3 = 0xCB,

    BISB2 = 0x88,
    BISB3 = 0x89,
    BISW2 = 0xA8,
    BISW3 = 0xA9,
    BISL2 = 0xC8,
    BISL3 = 0xC9,

    BITB = 0x93,
    BITW = 0xB3,
    BITL = 0xD3,

    CLRB = 0x94,
    CLRW = 0xB4,
    CLRL = 0xD4,
    CLRQ = 0x7C,
    CLRO = 0x7CFD,

    CMPB = 0x91,
    CMPW = 0xB1,
    CMPL = 0xD1,

    CVTBW = 0x99,
    CVTBL = 0x98,
    CVTWB = 0x33,
    CVTWL = 0x32,
    CVTLB = 0xF6,
    CVTLW = 0xF7,

    DECB = 0x97,
    DECW = 0xB7,
    DECL = 0xD7,

    DIVB2 = 0x86,
    DIVB3 = 0x87,
    DIVW2 = 0xA6,
    DIVW3 = 0xA7,
    DIVL2 = 0xC6,
    DIVL3 = 0xC7,

    EDIV = 0x7B,
    EMUL = 0x74,

    INCB = 0x96,
    INCW = 0xB6,
    INCL = 0xD6,

    MCOMB = 0x92,
    MCOMW = 0xB2,
    MCOML = 0xD2,

    MNEGB = 0x8E,
    MNEGW = 0xAE,
    MNEGL = 0xCE,

    MOVB = 0x90,
    MOVW = 0xB0,
    MOVL = 0xD0,
    MOVQ = 0x7D,
    MOVO = 0x7DFD,

    MOVZBW = 0x9B,
    MOVZBL = 0x9A,
    MOVZWL = 0x3C,

    MULB2 = 0x84,
    MULB3 = 0x85,
    MULW2 = 0xA4,
    MULW3 = 0xA5,
    MULL2 = 0xC4,
    MULL3 = 0xC5,

    PUSHL = 0xDD,

    ROTL = 0x9C,
    
    SBWC = 0xD9,

    SUBB2 = 0x82,
    SUBB3 = 0x83,
    SUBW2 = 0xA2,
    SUBW3 = 0xA3,
    SUBL2 = 0xC2,
    SUBL3 = 0xC3,

    TSTB = 0x95,
    TSTW = 0xB5,
    TSTL = 0xD5,
    
    XORB2 = 0x8C,
    XORB3 = 0x8D,
    XORW2 = 0xAC,
    XORW3 = 0xAD,
    XORL2 = 0xCC,
    XORL3 = 0xCD,

    MOVAB = 0x9E,
    MOVAW = 0x3E,
    MOVAL = 0xDE,
    MOVAQ = 0x7E,
    MOVAO = 0x7EFD,

    PUSHAB = 0x9F,
    PUSHAW = 0x3F,
    PUSHAL = 0xDF,
    PUSHAQ = 0x7F,
    PUSHAO = 0x7FFD,

    CMPV = 0xEC,
    CMPZV = 0xED,

    EXTV = 0xEE,
    EXTZV = 0xEF,

    FFC = 0xEB,
    FFS = 0xEA,

    INSV = 0xF0,

    ACBB = 0x9D,
    ACBW = 0x3D,
    ACBL = 0xF1,
    ACBF = 0x4F,
    ACBD = 0x6F, // Unsupported instruction, D_floating
    ACBG = 0x4FFD,
    ACBH = 0x6FFD, // Unsupported instruction, H_floating

    AOBLEQ = 0xF3,

    AOBLSS = 0xF2,

    BGTR = 0x14,
    BLEQ = 0x15,
    BNEQ = 0x12,
    BEQL = 0x13,
    BGEQ = 0x18,
    BLSS = 0x19,
    BGTRU = 0x1A,
    BLEQU = 0x1B,
    BVC = 0x1C,
    BVS = 0x1D,
    BCC = 0x1E,
    BCS = 0x1F,

    BBS = 0xE0,
    BBC = 0xE1,

    BBSS = 0xE2,
    BBCS = 0xE3,
    BBSC = 0xE4,
    BBCC = 0xE5,

    BBSSI = 0xE6,
    BBCCI = 0xE7,

    BLBS = 0xE8,
    BLBC = 0xE9,

    BRB = 0x11,
    BRW = 0x31,

    BSBB = 0x10,
    BSBW = 0x30,

    CASEB = 0x8F,
    CASEW = 0xAF,
    CASEL = 0xCF,

    JMP = 0x17,

    JSB = 0x16,

    RSB = 0x05,

    SOBGEQ = 0xF4,

    SOBGTR = 0xF5,

    CALLG = 0xFA,
    CALLS = 0xFB,
    RET = 0x04,

    BICPSW = 0xB9,
    BISPSW = 0xB8,

    BPT = 0x03,
    BUGW = 0xFEFF,
    BUGL = 0xFDFF,

    HALT = 0x00,

    INDEX = 0x0A,

    MOVPSL = 0xDC,

    NOP = 0x01,

    POPR = 0xBA,
    PUSHR = 0xBB,

    XFC = 0xFC,

    INSQHI = 0x5C,
    INSQTI = 0x5D,
    INSQUE = 0x0E, // Why are the queue instructions in the major 252 instructions? 
    REMQHI = 0x5E, // They'd be better put in one of the 3 extension prefixes...
    REMQTI = 0x5F,
    REMQUE = 0x0F,
}

/// GENERATED instruction field modes
impl InstructionType {
    pub fn from_instrid(instr: [u8;2]) -> Option<Self> {
        match instr[0] {
            0xFD | 0xFE | 0xFF => 
                InstructionType::from_u16(u16::from_le_bytes(instr)),
            v => InstructionType::from_u8(v),
        }
    }

    pub fn field_count(self) -> u32 {
        // don't repeat yourself.
        self.field_modes().len() as u32
    }

    pub fn field_modes(self) -> &'static [FieldMode] {
        use InstructionType::*;
        // Why a bunch of consts?
        // Pretty simple: Passing around a static slice doesn't require a memory allocation        
        const FM_NONE: &'static [FieldMode] =
            &[];
        
        const FM_R: &'static [FieldMode] =
            &[FieldMode::Read];
        const FM_W: &'static [FieldMode] =
            &[FieldMode::Write];
        const FM_M: &'static [FieldMode] =
            &[FieldMode::Modify];
        const FM_A: &'static [FieldMode] =
            &[FieldMode::Address];
        const FM_BB: &'static [FieldMode] =
            &[FieldMode::DataByte];
        const FM_BW: &'static [FieldMode] =
            &[FieldMode::DataWord];
        const FM_BL: &'static [FieldMode] =
            &[FieldMode::DataLong];

        const FM_RR: &'static [FieldMode] =
            &[FieldMode::Read, FieldMode::Read];
        const FM_RBB: &'static [FieldMode] =
            &[FieldMode::Read, FieldMode::DataByte];
        const FM_MBB: &'static [FieldMode] =
            &[FieldMode::Modify, FieldMode::DataByte];
        const FM_RM: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Modify];
        const FM_RW: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Write];
        const FM_AW: &'static [FieldMode] = 
            &[FieldMode::Address, FieldMode::Write];
        const FM_AA: &'static [FieldMode] = 
            &[FieldMode::Address, FieldMode::Address];
        const FM_RA: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Address];

        const FM_RRW: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Read, FieldMode::Write];
        const FM_RMBB: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Modify, FieldMode::DataByte];
        const FM_RVBB: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Bitfield, FieldMode::DataByte];

        const FM_RRWW: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Read, FieldMode::Write, FieldMode::Write];
        const FM_RRRW: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Read, FieldMode::Read, FieldMode::Write];
        const FM_RRVR: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Read, FieldMode::Bitfield, FieldMode::Read];
        const FM_RRVW: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Read, FieldMode::Bitfield, FieldMode::Write];
        const FM_RRRV: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Read, FieldMode::Read, FieldMode::Bitfield];

        const FM_RRMBW: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Read, FieldMode::Modify, FieldMode::DataWord];

        const FM_RRRRRW: &'static [FieldMode] =
            &[FieldMode::Read, FieldMode::Read, FieldMode::Read, FieldMode::Read, FieldMode::Read, FieldMode::Write];
        
        // >:(
        const FM_CASE: &'static [FieldMode] =
            &[FieldMode::Read, FieldMode::Read, FieldMode::Read, FieldMode::VariableLengthTable];

        match self {
            ADAWI => FM_RM,
            ADDB2 | ADDW2 | ADDL2 => FM_RM,
            ADDB3 | ADDW3 | ADDL3 => FM_RRW,
            ADWC => FM_RM,
            ASHL | ASHQ => FM_RRW,
            BICB2 | BICW2 | BICL2 => FM_RM,
            BICB3 | BICW3 | BICL3 => FM_RRW,
            BISB2 | BISW2 | BISL2 => FM_RM,
            BISB3 | BISW3 | BISL3 => FM_RRW,
            BITB | BITW | BITL => FM_RR,
            CLRB | CLRW | CLRL | CLRQ | CLRO => FM_W,
            CMPB | CMPW | CMPL => FM_RR,
            CVTBW | CVTBL | CVTWB | CVTWL | CVTLB | CVTLW => FM_RW,
            DECB | DECW | DECL => FM_M,
            DIVB2 | DIVW2 | DIVL2 => FM_RM,
            DIVB3 | DIVW3 | DIVL3 => FM_RRW,
            EDIV => FM_RRWW,
            EMUL => FM_RRRW,
            INCB | INCW | INCL => FM_M,
            MCOMB | MCOMW | MCOML => FM_RW,
            MNEGB | MNEGW | MNEGL => FM_RW,
            MOVB | MOVW | MOVL | MOVQ | MOVO => FM_RW,
            MOVZBW | MOVZBL | MOVZWL => FM_RW,
            MULB2 | MULW2 | MULL2 => FM_RM,
            MULB3 | MULW3 | MULL3 => FM_RRW,
            PUSHL => FM_R,
            ROTL => FM_RRW,
            SBWC => FM_RM,
            SUBB2 | SUBW2 | SUBL2 => FM_RM,
            SUBB3 | SUBW3 | SUBL3 => FM_RRW,
            TSTB | TSTW | TSTL => FM_R,
            XORB2 | XORW2 | XORL2 => FM_RM,
            XORB3 | XORW3 | XORL3 => FM_RRW,
            MOVAB | MOVAW | MOVAL | MOVAQ | MOVAO => FM_AW,
            PUSHAB | PUSHAW | PUSHAL | PUSHAQ | PUSHAO => FM_A,
            CMPV | CMPZV => FM_RRVR,
            EXTV | EXTZV => FM_RRVW,
            FFC | FFS => FM_RRVW,
            INSV => FM_RRRV,
            ACBB | ACBW | ACBL | ACBF | ACBD | ACBG | ACBH => FM_RRMBW,
            AOBLEQ => FM_RMBB,
            AOBLSS => FM_RMBB,
            BGTR | BLEQ | BNEQ | BEQL | BGEQ | BLSS 
            | BGTRU | BLEQU | BVC | BVS | BCC | BCS => FM_BB,
            BBS | BBC => FM_RVBB,
            BBSS | BBCS | BBSC | BBCC => FM_RVBB,
            BBSSI | BBCCI => FM_RVBB,
            BLBS | BLBC => FM_RBB,
            BRB | BSBB => FM_BB,
            BRW | BSBW => FM_BW,
            CASEB | CASEW | CASEL => FM_CASE,
            JMP => FM_A,
            JSB => FM_A,
            RSB => FM_NONE,
            SOBGEQ => FM_MBB,
            SOBGTR => FM_MBB,
            CALLG => FM_AA,
            CALLS => FM_RA,
            RET => FM_NONE,
            BICPSW => FM_R,
            BISPSW => FM_R,
            BPT => FM_NONE,
            BUGW => FM_BW,
            BUGL => FM_BL,
            HALT => FM_NONE,
            INDEX => FM_RRRRRW,
            MOVPSL => FM_W,
            NOP => FM_NONE,
            POPR => FM_R,
            PUSHR => FM_R,
            XFC => FM_NONE,
            INSQHI => FM_AA,
            INSQTI => FM_AA,
            INSQUE => FM_AA,
            REMQHI => FM_AW,
            REMQTI => FM_AW,
            REMQUE => FM_AW,
        }
    }
}
