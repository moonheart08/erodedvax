use num_derive::*;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::ervax::cpu::instrs::{
    FieldMode,
    OperandWidth,
};



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
    EMUL = 0x7A,

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

    // Queue instructions

    INSQHI = 0x5C,
    INSQTI = 0x5D,
    INSQUE = 0x0E, // Why are the queue instructions in the major 252 instructions? 
    REMQHI = 0x5E, // They'd be better put in one of the 3 extension prefixes...
    REMQTI = 0x5F,
    REMQUE = 0x0F,

    //Floating point

    ADDF2 = 0x40,
    ADDF3 = 0x41,
    ADDD2 = 0x60, // Unsupported
    ADDD3 = 0x61, // Unsupported
    ADDG2 = 0x40FD,
    ADDG3 = 0x41FD,
    ADDH2 = 0x60FD, // Unsupported
    ADDH3 = 0x61FD, // Unsupported

    CMPF = 0x51,
    CMPD = 0x71, // Unsupported
    CMPG = 0x51FD,
    CMPH = 0x71FD, // Unsupported

    // Floating point convert instructions.

    CVTBF = 0x4C,
    CVTWF = 0x4D,
    CVTLF = 0x4E,

    CVTBD = 0x6C, // Unsupported
    CVTWD = 0x6D, // Unsupported
    CVTLD = 0x6E, // Unsupported

    CVTBG = 0x4CFD,
    CVTWG = 0x4DFD,
    CVTLG = 0x4EFD,

    CVTBH = 0x6CFD, // Unsupported
    CVTWH = 0x6DFD, // Unsupported
    CVTLH = 0x6EFD, // Unsupported

    CVTFB = 0x48,
    CVTFW = 0x49,
    CVTFL = 0x4A,
    CVTRFL = 0x4B,

    CVTDB = 0x68, // Unsupported
    CVTDW = 0x69, // Unsupported
    CVTDL = 0x6A, // Unsupported
    CVTRDL = 0x6B, // Unsupported

    CVTGB = 0x48FD,
    CVTGW = 0x49FD,
    CVTGL = 0x4AFD,
    CVTRGL = 0x4BFD,

    CVTHB = 0x68FD, // Unsupported
    CVTHW = 0x69FD, // Unsupported
    CVTHL = 0x6AFD, // Unsupported
    CVTRHL = 0x6BFD, // Unsupported

    CVTFD = 0x56, // Unsupported
    CVTFG = 0x99FD,
    CVTFH = 0x98FD, // Unsupported

    CVTDF = 0x76, // Unsupported
    CVTDH = 0x32FD, // Unsupported

    CVTGF = 0x33FD,
    CVTGH = 0x56FD, // Unsupported

    CVTHF = 0xF6FD, // Unsupported
    CVTHD = 0xF7FD, // Unsupported
    CVTHG = 0x76FD, // Unsupported

    // Rest of floating point.

    DIVF2 = 0x46,
    DIVF3 = 0x47,
    DIVD2 = 0x66, // Unsupported
    DIVD3 = 0x67, // Unsupported
    DIVG2 = 0x46FD,
    DIVG3 = 0x47FD,
    DIVH2 = 0x66FD, // Unsupported
    DIVH3 = 0x67FD, // Unsupported

    EMODF = 0x54,
    EMODD = 0x74, // Unsupported
    EMODG = 0x54FD,
    EMODH = 0x74FD, // Unsupported

    MNEGF = 0x52,
    MNEGD = 0x72, // Unsupported
    MNEGG = 0x52FD,
    MNEGH = 0x72FD, // Unsupported

    MOVF = 0x50,
    MOVD = 0x70, // Unsupported
    MOVG = 0x50FD,
    MOVH = 0x70FD, // Unsupported

    MULF2 = 0x44,
    MULF3 = 0x45,
    MULD2 = 0x64, // Unsupported
    MULD3 = 0x65, // Unsupported
    MULG2 = 0x44FD,
    MULG3 = 0x45FD,
    MULH2 = 0x64FD, // Unsupported
    MULH3 = 0x65FD, // Unsupported

    POLYF = 0x55, // All POLY instructions are unimplemented for now. Not top priority.
    POLYD = 0x75, // Unsupported
    POLYG = 0x55FD,
    POLYH = 0x75FD, // Unsupported

    SUBF2 = 0x42,
    SUBF3 = 0x43,
    SUBD2 = 0x62, // Unsupported
    SUBD3 = 0x63, // Unsupported
    SUBG2 = 0x42FD,
    SUBG3 = 0x43FD,
    SUBH2 = 0x62FD, // Unsupported
    SUBH3 = 0x63FD, // Unsupported

    TSTF = 0x53,
    TSTD = 0x73,
    TSTG = 0x53FD,
    TSTH = 0x73FD,
    
    /// Instructions for exceptions and interrupts.
    
    REI = 0x02,

    CHMK = 0xBC,
    CHME = 0xBD,
    CHMS = 0xBE,
    CHMU = 0xBF,

    /// Instructions for processes.

    LDPCTX = 0x06,
    SVPCTX = 0x07,

    MTPR = 0xDA,
    MFPR = 0xDB,
}

impl InstructionType {
    #[inline]
    pub fn from_instrid<I>(bytes: &mut I) -> Option<Self> 
        where I: Iterator<Item = u8>
    {
        if let Some(b) = bytes.next() {
            match b {
                0xFD | 0xFE | 0xFF => {
                    if let Some(c) = bytes.next() {
                        InstructionType::from_u16(u16::from_le_bytes([b,c]))
                    } else {
                        None
                    }
                }
                v => InstructionType::from_u8(v),
            }
        } else {
            None
        }
    }

    #[inline]
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
        const FM_D: &'static [FieldMode] =
            &[FieldMode::Data];
        const FM_BL: &'static [FieldMode] =
            &[FieldMode::Data];

        const FM_RR: &'static [FieldMode] =
            &[FieldMode::Read, FieldMode::Read];
        const FM_RD: &'static [FieldMode] =
            &[FieldMode::Read, FieldMode::Data];
        const FM_MD: &'static [FieldMode] =
            &[FieldMode::Modify, FieldMode::Data];
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
        const FM_RRA: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Read, FieldMode::Address];
        const FM_RMD: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Modify, FieldMode::Data];
        const FM_RVD: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Bitfield, FieldMode::Data];

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

        const FM_RRMD: &'static [FieldMode] = 
            &[FieldMode::Read, FieldMode::Read, FieldMode::Modify, FieldMode::Data];

        const FM_RRRWW: &'static [FieldMode] =
            &[FieldMode::Read, FieldMode::Read, FieldMode::Read, FieldMode::Write, FieldMode::Write];

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
            ACBB | ACBW | ACBL | ACBF | ACBD | ACBG | ACBH => FM_RRMD,
            AOBLEQ => FM_RMD,
            AOBLSS => FM_RMD,
            BGTR | BLEQ | BNEQ | BEQL | BGEQ | BLSS 
            | BGTRU | BLEQU | BVC | BVS | BCC | BCS => FM_D,
            BBS | BBC => FM_RVD,
            BBSS | BBCS | BBSC | BBCC => FM_RVD,
            BBSSI | BBCCI => FM_RVD,
            BLBS | BLBC => FM_RD,
            BRB | BSBB => FM_D,
            BRW | BSBW => FM_D,
            CASEB | CASEW | CASEL => FM_CASE,
            JMP => FM_A,
            JSB => FM_A,
            RSB => FM_NONE,
            SOBGEQ => FM_MD,
            SOBGTR => FM_MD,
            CALLG => FM_AA,
            CALLS => FM_RA,
            RET => FM_NONE,
            BICPSW => FM_R,
            BISPSW => FM_R,
            BPT => FM_NONE,
            BUGW => FM_D,
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
            ADDF2 | ADDD2 | ADDG2 | ADDH2 => FM_RM,
            ADDF3 | ADDD3 | ADDG3 | ADDH3 => FM_RRW,
            CMPF | CMPD | CMPG | CMPH => FM_RR,
            // oh god
            CVTBF | CVTWF | CVTLF | CVTBD | CVTWD | CVTLD | CVTBG | CVTWG | CVTLG
            | CVTBH | CVTWH | CVTLH | CVTFB | CVTFW | CVTFL | CVTRFL | CVTDB | CVTDW
            | CVTDL | CVTRDL | CVTGB | CVTGW | CVTGL | CVTRGL | CVTHB | CVTHW | CVTHL
            | CVTFD | CVTFG | CVTFH | CVTDF | CVTDH | CVTGF | CVTGH
            | CVTHF | CVTHD | CVTHG | CVTRHL => FM_RW,
            DIVF2 | DIVD2 | DIVG2 | DIVH2 => FM_RM,
            DIVF3 | DIVD3 | DIVG3 | DIVH3 => FM_RRW,
            EMODF | EMODD | EMODG | EMODH => FM_RRRWW,
            MNEGF | MNEGD | MNEGG | MNEGH => FM_RW,
            MOVF | MOVD | MOVG | MOVH => FM_RW,
            MULF2 | MULD2 | MULG2 | MULH2 => FM_RM,
            MULF3 | MULD3 | MULG3 | MULH3 => FM_RRW,
            POLYF | POLYD | POLYG | POLYH => FM_RRA,
            SUBF2 | SUBD2 | SUBG2 | SUBH2 => FM_RM,
            SUBF3 | SUBD3 | SUBG3 | SUBH3 => FM_RRW,
            TSTF | TSTD | TSTG | TSTH => FM_R,
            REI => FM_NONE,
            CHMK | CHME | CHMS | CHMU => FM_R,
            LDPCTX => FM_NONE,
            SVPCTX => FM_NONE,
            MTPR => FM_RR,
            MFPR => FM_RW,
        }
    }

    pub fn field_widths(self) -> &'static [OperandWidth] {
        use InstructionType::*;
        use OperandWidth as OW;

        const FW_NONE: &'static [OperandWidth] =
            &[];

        const FW_B: &'static [OperandWidth] = 
            &[OW::Byte];
        const FW_W: &'static [OperandWidth] = 
            &[OW::Word];
        const FW_L: &'static [OperandWidth] = 
            &[OW::Longword];
        const FW_Q: &'static [OperandWidth] = 
            &[OW::Quadword];
        const FW_O: &'static [OperandWidth] = 
            &[OW::Octaword];
        
        const FW_BB: &'static [OperandWidth] = 
            &[OW::Byte, OW::Byte];
        const FW_BW: &'static [OperandWidth] = 
            &[OW::Byte, OW::Word];
        const FW_BL: &'static [OperandWidth] = 
            &[OW::Byte, OW::Longword];
        const FW_BQ: &'static [OperandWidth] = 
            &[OW::Byte, OW::Quadword];
        const FW_BO: &'static [OperandWidth] = 
            &[OW::Byte, OW::Octaword];
        
        const FW_WB: &'static [OperandWidth] = 
            &[OW::Word, OW::Byte];
        const FW_WW: &'static [OperandWidth] = 
            &[OW::Word, OW::Word];
        const FW_WL: &'static [OperandWidth] = 
            &[OW::Word, OW::Longword];
        const FW_WQ: &'static [OperandWidth] = 
            &[OW::Word, OW::Quadword];
        const FW_WO: &'static [OperandWidth] = 
            &[OW::Word, OW::Quadword];
        
        const FW_LB: &'static [OperandWidth] = 
            &[OW::Longword, OW::Byte];
        const FW_LW: &'static [OperandWidth] = 
            &[OW::Longword, OW::Word];
        const FW_LL: &'static [OperandWidth] = 
            &[OW::Longword, OW::Longword];
        const FW_LQ: &'static [OperandWidth] = 
            &[OW::Longword, OW::Quadword];
        const FW_LO: &'static [OperandWidth] = 
            &[OW::Longword, OW::Octaword];
        
        const FW_QB: &'static [OperandWidth] = 
            &[OW::Quadword, OW::Byte];
        const FW_QW: &'static [OperandWidth] = 
            &[OW::Quadword, OW::Word];
        const FW_QL: &'static [OperandWidth] = 
            &[OW::Quadword, OW::Longword];
        const FW_QQ: &'static [OperandWidth] = 
            &[OW::Quadword, OW::Quadword];
        const FW_QO: &'static [OperandWidth] = 
            &[OW::Quadword, OW::Octaword];
        
        const FW_OB: &'static [OperandWidth] = 
            &[OW::Octaword, OW::Byte];
        const FW_OW: &'static [OperandWidth] = 
            &[OW::Octaword, OW::Word];
        const FW_OL: &'static [OperandWidth] = 
            &[OW::Octaword, OW::Longword];
        const FW_OQ: &'static [OperandWidth] = 
            &[OW::Octaword, OW::Longword];
        const FW_OO: &'static [OperandWidth] = 
            &[OW::Octaword, OW::Octaword];


        const FW_BBB: &'static [OperandWidth] = 
            &[OW::Byte, OW::Byte, OW::Byte];
        const FW_BLL: &'static [OperandWidth] = 
            &[OW::Byte, OW::Longword, OW::Longword];
        const FW_BQQ: &'static [OperandWidth] = 
            &[OW::Byte, OW::Quadword, OW::Quadword];
        const FW_WWW: &'static [OperandWidth] = 
            &[OW::Word, OW::Word, OW::Word];
        const FW_LLL: &'static [OperandWidth] = 
            &[OW::Longword, OW::Longword, OW::Longword];
        const FW_LLB: &'static [OperandWidth] = 
            &[OW::Longword, OW::Longword, OW::Byte];
        const FW_LBB: &'static [OperandWidth] = 
            &[OW::Longword, OW::Byte, OW::Byte];
        const FW_QQQ: &'static [OperandWidth] = 
            &[OW::Quadword, OW::Quadword, OW::Quadword];
        const FW_OOO: &'static [OperandWidth] = 
            &[OW::Octaword, OW::Octaword, OW::Octaword];

        const FW_LLBB: &'static [OperandWidth] =
            &[OW::Longword, OW::Longword, OW::Byte, OW::Byte];
        const FW_LBBL: &'static [OperandWidth] =
            &[OW::Longword, OW::Byte, OW::Byte, OW::Longword];

        match self {
            ADAWI => FW_WW,
            ADDB2 => FW_BB,
            ADDB3 => FW_BBB,
            ADDW2 => FW_WW,
            ADDW3 => FW_WWW,
            ADDL2 => FW_LL,
            ADDL3 => FW_LLL,
            ADWC => FW_LL,
            ASHL => FW_BLL,
            ASHQ => FW_BQQ,
            BICB2 => FW_BB,
            BICB3 => FW_BBB,
            BICW2 => FW_WW,
            BICW3 => FW_WWW,
            BICL2 => FW_LL,
            BICL3 => FW_LLL,
            BISB2 => FW_BB,
            BISB3 => FW_BBB,
            BISW2 => FW_WW,
            BISW3 => FW_WWW,
            BISL2 => FW_LL,
            BISL3 => FW_LLL,
            BITB => FW_BB,
            BITW => FW_WW,
            BITL => FW_LL,
            CLRB => FW_B,
            CLRW => FW_W,
            CLRL => FW_L,
            CLRQ => FW_Q,
            CLRO => FW_O,
            CMPB => FW_BB,
            CMPW => FW_WW,
            CMPL => FW_LL,
            CVTBW => &[OW::Byte, OW::Word],
            CVTBL => &[OW::Byte, OW::Longword],
            CVTWB => &[OW::Word, OW::Byte],
            CVTWL => &[OW::Word, OW::Longword],
            CVTLB => &[OW::Longword, OW::Byte],
            CVTLW => &[OW::Longword, OW::Word],
            DECB => FW_B,
            DECW => FW_W,
            DECL => FW_L,
            DIVB2 => FW_BB,
            DIVB3 => FW_BBB,
            DIVW2 => FW_WW,
            DIVW3 => FW_WWW,
            DIVL2 => FW_LL,
            DIVL3 => FW_LLL,
            EDIV => &[OW::Longword, OW::Quadword, OW::Longword, OW::Longword],
            EMUL => &[OW::Longword, OW::Longword, OW::Longword, OW::Quadword],
            INCB => FW_B,
            INCW => FW_W,
            INCL => FW_L,
            MCOMB => FW_BB,
            MCOMW => FW_WW,
            MCOML => FW_LL,
            MNEGB => FW_BB,
            MNEGW => FW_WW,
            MNEGL => FW_LL,
            MOVB => FW_BB,
            MOVW => FW_WW,
            MOVL => FW_LL,
            MOVQ => FW_QQ,
            MOVO => FW_OO,
            MOVZBW => &[OW::Byte, OW::Word],
            MOVZBL => &[OW::Byte, OW::Longword],
            MOVZWL => &[OW::Word, OW::Longword],
            MULB2 => FW_BB,
            MULB3 => FW_BBB,
            MULW2 => FW_WW,
            MULW3 => FW_WWW,
            MULL2 => FW_LL,
            MULL3 => FW_LLL,
            PUSHL => FW_L,
            ROTL => FW_BLL,
            SBWC => FW_LL,
            SUBB2 => FW_BB,
            SUBB3 => FW_BBB,
            SUBW2 => FW_WW,
            SUBW3 => FW_WWW,
            SUBL2 => FW_LL,
            SUBL3 => FW_LLL,
            TSTB => FW_B,
            TSTW => FW_W,
            TSTL => FW_L,
            XORB2 => FW_BB,
            XORB3 => FW_BBB,
            XORW2 => FW_WW,
            XORW3 => FW_WWW,
            XORL2 => FW_LL,
            XORL3 => FW_LLL,
            MOVAB => FW_BL,
            MOVAW => FW_WL,
            MOVAL => FW_LL,
            MOVAQ => FW_QL,
            MOVAO => FW_OL,
            PUSHAB => FW_B,
            PUSHAW => FW_W,
            PUSHAL => FW_L,
            PUSHAQ => FW_Q,
            PUSHAO => FW_O,
            CMPV => FW_LBBL,
            CMPZV => FW_LBBL,
            EXTV => FW_LBBL,
            EXTZV => FW_LBBL,
            FFC => FW_LBBL,
            FFS => FW_LBBL,
            INSV => FW_LLBB,
            ACBB => &[OW::Byte, OW::Byte, OW::Byte, OW::Word],
            ACBW => &[OW::Word, OW::Word, OW::Word, OW::Word],
            ACBL => &[OW::Longword, OW::Longword, OW::Longword, OW::Word],
            ACBF => &[OW::Longword, OW::Longword, OW::Longword, OW::Word],
            ACBD => &[OW::Quadword, OW::Quadword, OW::Quadword, OW::Word],
            ACBG => &[OW::Quadword, OW::Quadword, OW::Quadword, OW::Word],
            ACBH => &[OW::Octaword, OW::Octaword, OW::Octaword, OW::Word],
            AOBLEQ => FW_LLB,
            AOBLSS => FW_LLB,
            BGTR | BLEQ | BNEQ | BEQL | BGEQ | BLSS 
            | BGTRU | BLEQU | BVC | BVS | BCC | BCS => FW_B,
            BBS => FW_LBB,
            BBC => FW_LBB,
            BBSS => FW_LBB,
            BBCS => FW_LBB,
            BBSC => FW_LBB,
            BBCC => FW_LBB,
            BBSSI => FW_LBB,
            BBCCI => FW_LBB,
            BLBS => FW_LB,
            BLBC => FW_LB,
            BRB => FW_B,
            BRW => FW_W,
            BSBB => FW_B,
            BSBW => FW_W,
            CASEB => &[OW::Byte, OW::Byte, OW::Byte, OW::Word],
            CASEW => &[OW::Word, OW::Word, OW::Word, OW::Word],
            CASEL => &[OW::Longword, OW::Longword, OW::Longword, OW::Word],
            JMP => FW_B,
            JSB => FW_B,
            RSB => FW_NONE,
            SOBGEQ => FW_LB,
            SOBGTR => FW_LB,
            CALLG => FW_BB,
            CALLS => FW_LB,
            RET => FW_NONE,
            BICPSW => FW_W,
            BISPSW => FW_W,
            BPT => FW_NONE,
            BUGW => FW_W,
            BUGL => FW_L,
            HALT => FW_NONE,
            INDEX => &[OW::Longword, OW::Longword, OW::Longword, OW::Longword, OW::Longword, OW::Longword],
            MOVPSL => FW_L,
            NOP => FW_NONE,
            POPR => FW_W,
            PUSHR => FW_W,
            XFC => FW_NONE,
            INSQHI => FW_BQ,
            INSQTI => FW_BQ,
            INSQUE => FW_BB,
            REMQHI => FW_QL,
            REMQTI => FW_QL,
            REMQUE => FW_BL,
            ADDF2 => FW_LL,
            ADDF3 => FW_LLL,
            ADDD2 => FW_QQ,
            ADDD3 => FW_QQQ,
            ADDG2 => FW_QQ,
            ADDG3 => FW_QQQ,
            ADDH2 => FW_OO,
            ADDH3 => FW_OOO,
            CMPF => FW_LL,
            CMPD => FW_QQ,
            CMPG => FW_QQ,
            CMPH => FW_OO,
            // C V T AAAAA
            CVTBF => FW_BL,
            CVTWF => FW_WL,
            CVTLF => FW_LL,
            CVTBD | CVTBG => FW_BQ,
            CVTWD | CVTWG => FW_WQ,
            CVTLD | CVTLG => FW_LQ,
            CVTBH => FW_BO,
            CVTWH => FW_WO,
            CVTLH => FW_LO,
            CVTFB => FW_LB,
            CVTFW => FW_LW,
            CVTFL | CVTRFL => FW_LL,
            CVTDB | CVTGB => FW_QB,
            CVTDW | CVTGW => FW_QW,
            CVTDL | CVTGL | CVTRDL | CVTRGL => FW_QL,
            CVTHB => FW_OB,
            CVTHW => FW_OW,
            CVTHL | CVTRHL => FW_OL,
            CVTFD | CVTFG => FW_LQ,
            CVTFH => FW_LO,
            CVTDF | CVTGF => FW_QL,
            CVTDH | CVTGH => FW_QO,
            CVTHF => FW_OL,
            CVTHD | CVTHG => FW_OQ,
            DIVF2 => FW_LL,
            DIVF3 => FW_LLL,
            DIVD2 | DIVG2 => FW_QQ,
            DIVD3 | DIVG3 => FW_QQQ,
            DIVH2 => FW_OO,
            DIVH3 => FW_OOO,
            EMODF => &[OW::Longword, OW::Byte, OW::Longword, OW::Longword, OW::Longword],
            EMODD => &[OW::Quadword, OW::Byte, OW::Quadword, OW::Longword, OW::Quadword],
            EMODG => &[OW::Quadword, OW::Word, OW::Quadword, OW::Longword, OW::Quadword],
            EMODH => &[OW::Octaword, OW::Byte, OW::Octaword, OW::Octaword, OW::Octaword],
            MNEGF => FW_LL,
            MNEGD | MNEGG => FW_QQ,
            MNEGH => FW_OO,
            MOVF => FW_LL,
            MOVD | MOVG => FW_QQ,
            MOVH => FW_OO,
            MULF2 => FW_LL,
            MULF3 => FW_LLL,
            MULD2 | MULG2 => FW_QQ,
            MULD3 | MULG3 => FW_QQQ,
            MULH2 => FW_OO,
            MULH3 => FW_OOO,
            POLYF => &[OW::Longword, OW::Word, OW::Byte],
            POLYD | POLYG => &[OW::Quadword, OW::Word, OW::Byte],
            POLYH => &[OW::Octaword, OW::Word, OW::Byte],
            SUBF2 => FW_LL,
            SUBF3 => FW_LLL,
            SUBD2 | SUBG2 => FW_QQ,
            SUBD3 | SUBG3 => FW_QQQ,
            SUBH2 => FW_OO,
            SUBH3 => FW_OOO,
            TSTF => FW_L,
            TSTD | TSTG => FW_Q,
            TSTH => FW_O,
            REI => FW_NONE,
            CHMK | CHME | CHMS | CHMU => FW_W,
            LDPCTX | SVPCTX => FW_NONE,
            MFPR | MTPR => FW_LL,
        }
    }
}


mod tests {
    use crate::ervax::cpu::{
        instrs::{
            OperandMode,
            OperandParseError,
            OperandWidth,
            OperandIter,
            InstructionType,
            decode_instr,
        },
        RegID,
    };

    #[test]
    /// Tests to make sure field_modes and field_widths return the same length arrays for all instructions
    fn all_operand_list_lens_equal() {

        for i in 0..252 {
            let v = vec![i as u8];
            let iter = &mut (v.iter().map(|x| *x));
            if let Some(i) = InstructionType::from_instrid(iter) {
                if i.field_widths().len() != i.field_modes().len() {
                    panic!("Instruction {:?} has mismatched field modes/width lengths!", i);
                }
            }
        }

        for i in 252..256 {
            for j in 0..256 {
                let v = vec![i as u8, j as u8];
                let v = vec![i as u8];
                let iter = &mut (v.iter().map(|x| *x));
                if let Some(i) = InstructionType::from_instrid(iter) {
                    if i.field_widths().len() != i.field_modes().len() {
                        panic!("Instruction {:?} has mismatched field modes/width lengths!", i);
                    }
                }
            }
        }
    }
}