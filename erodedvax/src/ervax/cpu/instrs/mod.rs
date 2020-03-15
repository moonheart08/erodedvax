use num_derive::*;
use num_traits::{FromPrimitive, ToPrimitive};

mod instrtypes;

pub use instrtypes::*;

mod instrparse;

pub use instrparse::*;

use crate::ervax::cpu::{RegID};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OperandWidth {
    Byte, // u8
    Word, // 16
    Longword, // u32
    Quadword, // u64
    Octaword, // u128
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    DataByte(u8),
    DataWord(u16),
    DataLong(u32),
}

impl OperandMode {
    pub fn is_valid_indexed(&self) -> bool {
        use OperandMode::*;
        match self {
            Literal(_) | Indexed(_) | Register(_) |
            AutoDecrement(_) | AutoIncrement(_) | AutoIncrementDeferred(_)
            | Immediate8(_) | Immediate16(_) | Immediate32(_) | Immediate64(_) | Immediate128(_) => 
                false,
            _ => true,
        }
    }

    pub fn is_valid_in_fieldmode(&self, mode: FieldMode) -> bool {
        use OperandMode::*;
        match mode {
            FieldMode::Read => {
                return true; // all modes supported
            },
            FieldMode::Write | FieldMode::Modify => {
                match self {
                    Literal(_) | Immediate8(_) | Immediate16(_) | Immediate32(_) | Immediate64(_) | Immediate128(_) =>
                        false,
                    _ => true,
                }
            }
            FieldMode::Address => {
                match self {
                    Literal(_) | Register(_) => false,
                    _ => true,
                }
            }
            FieldMode::Bitfield => {
                match self {
                    Literal(_) => false,
                    _ => true,
                }
            }
            _ => unimplemented!("Function not applicable for Data and VariableWidthTable field modes.")
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    if d.len() != 8 { return None }
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
    pub fn read_operand<I>(bytes: &mut I, width: OperandWidth, allow_indexed: bool) -> Result<OperandMode, OperandParseError>
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
                    if !allow_indexed {
                        return Err(OperandParseError::InvalidMode);
                    }

                    let submode = OperandMode::read_operand::<I>(bytes, width, false);

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FieldMode {
    Read,
    Modify,
    Write,
    Address,
    Bitfield,
    Data,
    VariableLengthTable, // CASE why.
}

mod tests {
    use crate::ervax::cpu::{
        instrs::{
            OperandMode,
            OperandParseError,
            OperandWidth,
        },
        RegID,
    };

    #[test] 
    fn decode_literal() {
        let literal: Vec<u8> = vec![0x02];
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Byte, true);
        assert_eq!(r, Ok(OperandMode::Literal(2)));
    }

    #[test] 
    fn decode_register() {
        let literal: Vec<u8> = vec![0x55];
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Byte, true);
        assert_eq!(r, Ok(OperandMode::Register(RegID(5))));
    }

    #[test] 
    fn decode_deferred_register() {
        let literal: Vec<u8> = vec![0x65];
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Byte, true);
        assert_eq!(r, Ok(OperandMode::RegisterDeferred(RegID(5))));
    }

    #[test] 
    fn decode_immediate8() {
        let literal: Vec<u8> = vec![0x8F, 0x02];
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Byte, true);
        assert_eq!(r, Ok(OperandMode::Immediate8(2)));
    }

    #[test] 
    fn decode_immediate16() {
        let mut literal: Vec<u8> = vec![0x8F];
        let mut x: Vec<u8> = (&(2 as u16).to_le_bytes())[..].into();
        literal.append(&mut x);
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Word, true);
        assert_eq!(r, Ok(OperandMode::Immediate16(2)));
    }

    #[test] 
    fn decode_immediate32() {
        let mut literal: Vec<u8> = vec![0x8F];
        let mut x: Vec<u8> = (&(2 as u32).to_le_bytes())[..].into();
        literal.append(&mut x);
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Longword, true);
        assert_eq!(r, Ok(OperandMode::Immediate32(2)));
    }

    #[test] 
    fn decode_immediate64() {
        let mut literal: Vec<u8> = vec![0x8F];
        let mut x: Vec<u8> = (&(2 as u64).to_le_bytes())[..].into();
        literal.append(&mut x);
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Quadword, true);
        assert_eq!(r, Ok(OperandMode::Immediate64(2)));
    }

    #[test] 
    fn decode_immediate128() {
        let mut literal: Vec<u8> = vec![0x8F];
        let mut x: Vec<u8> = (&(2 as u128).to_le_bytes())[..].into();
        literal.append(&mut x);
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Octaword, true);
        assert_eq!(r, Ok(OperandMode::Immediate128(2)));
    }

    #[test]
    fn decode_autoincrement() {
        let literal: Vec<u8> = vec![0x85];
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Byte, true);
        assert_eq!(r, Ok(OperandMode::AutoIncrement(RegID(5))));
    }

    #[test]
    fn decode_autoincrement_deferred() {
        let literal: Vec<u8> = vec![0x95];
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Byte, true);
        assert_eq!(r, Ok(OperandMode::AutoIncrementDeferred(RegID(5))));
    }

    #[test]
    fn decode_absolute() {
        let mut literal: Vec<u8> = vec![0x9F];
        let mut x: Vec<u8> = (&(0x1234_5678 as u32).to_le_bytes())[..].into();
        literal.append(&mut x);
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Byte, true);
        assert_eq!(r, Ok(OperandMode::Absolute(0x1234_5678)));
    }

    // -----
    // Operand decoding failure tests
    // -----

    #[test]
    fn doubly_indexed() {
        let literal: Vec<u8> = vec![0x40, 0x40];
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Byte, true);
        assert_eq!(r, Err(OperandParseError::InvalidMode));
    }

    #[test] 
    fn decode_immediate8_not_enough_bytes() {
        let literal: Vec<u8> = vec![0x8F];
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Byte, true);
        assert_eq!(r, Err(OperandParseError::OutOfBytes));
    }

    #[test]
    fn decode_no_bytes() {
        let literal: Vec<u8> = vec![];
        let iter = &mut literal.iter().map(|x| *x);

        let r = OperandMode::read_operand(iter, OperandWidth::Byte, true);
        assert_eq!(r, Err(OperandParseError::OutOfBytes));
    }
}