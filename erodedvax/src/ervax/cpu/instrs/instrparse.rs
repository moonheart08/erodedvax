use crate::ervax::cpu::instrs::{
    FieldMode,
    OperandWidth,
    OperandMode,
    OperandParseError,
    InstructionType,
    get_u16_from_stream,
    get_u32_from_stream,
};

pub struct OperandIter<'a, I: Iterator> 
    where I: Iterator<Item = u8>
{
    field_id: u32,
    fm: &'static [FieldMode],
    fw: &'static [OperandWidth],
    done: bool,
    bytes: &'a mut I,
}

impl<'a, I: Iterator> OperandIter<'a, I> 
    where I: Iterator<Item = u8>
{
    pub fn from_instr<'b>(inst: InstructionType, bytes: &'b mut I) -> OperandIter<'b, I> {
        OperandIter {
            field_id: 0,
            fm: inst.field_modes(),
            fw: inst.field_widths(),
            done: false,
            bytes,
        }
    }

    pub(crate) fn from_raw<'b>(fm: &'static [FieldMode], fw: &'static [OperandWidth], bytes: &'b mut I) -> OperandIter<'b, I> {
        OperandIter {
            field_id: 0,
            fm,
            fw,
            done: false,
            bytes,
        }
    }

    /// Consumes the OperandIter and returns the bytes iter it was created with, and the field it was on.
    pub fn destructure(self) -> (&'a mut I, u32) {
        return (self.bytes, self.field_id);
    }
}

impl<'a, I: Iterator> Iterator for OperandIter<'a, I>
    where I: Iterator<Item = u8>
{
    type Item = (Result<OperandMode, OperandParseError>, OperandWidth);

    fn next(&mut self) -> Option<Self::Item> {
        let curfield = self.field_id as usize;
        
        if self.done {
            return None;
        }

        if curfield >= self.fm.len() {
            self.done = true;
            return None;
        }

        let curwidth = self.fw[curfield];

        match self.fm[curfield] {
            FieldMode::Data => {
                match self.fw[curfield] {
                    OperandWidth::Byte => {
                        if let Some(next) = self.bytes.next() {
                            return Some((Ok(OperandMode::DataByte(next)), curwidth));
                        }
                        return Some((Err(OperandParseError::OutOfBytes), curwidth));
                    }
                    OperandWidth::Word => {
                        match get_u16_from_stream(self.bytes) {
                            Some(v) => return Some((Ok(OperandMode::DataWord(v)), curwidth)),
                            None => return Some((Err(OperandParseError::OutOfBytes), curwidth)),
                        }
                    }
                    OperandWidth::Longword => {
                        match get_u32_from_stream(self.bytes) {
                            Some(v) => return Some((Ok(OperandMode::DataLong(v)), curwidth)),
                            None => return Some((Err(OperandParseError::OutOfBytes), curwidth)),
                        }
                    }
                    _ => unimplemented!("Quadword and Octaword data width was not needed at time of implementation")
                }
            },
            FieldMode::VariableLengthTable => {
                return None;
            },
            v => {
                let opres = OperandMode::read_operand(self.bytes, curwidth, true);

                self.field_id += 1;
                if let Ok(om) = opres 
                {
                    if om.is_valid_in_fieldmode(v) {
                        return Some((Ok(om), curwidth));
                    } else {
                        return Some((Err(OperandParseError::InvalidMode), curwidth));
                    }
                } else {
                    return Some((opres, curwidth));
                }
            }
        }
    }
}

pub fn decode_instr<'a, I>(bytes: &'a mut I) -> (InstructionType, OperandIter<'a, I>)
    where I: Iterator<Item = u8>
{
    let instr = InstructionType::from_instrid(bytes).unwrap();

    (instr, OperandIter::from_instr(instr, bytes))
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
    fn decode_addl2_imm_reg() {
        let op = vec![0x80, 0x8F, 0x02, 0x51];
        let iter = &mut op.iter().map(|x| *x);

        let (instr, mut operiter) = decode_instr(iter);
        assert_eq!(instr, InstructionType::ADDB2);
        assert_eq!(operiter.next().unwrap(), (Ok(OperandMode::Immediate8(2)), OperandWidth::Byte));
        assert_eq!(operiter.next().unwrap(), (Ok(OperandMode::Register(RegID(1))), OperandWidth::Byte));
    }

    #[test]
    fn decode_bugw() {
        let op = vec![0xFF, 0xFE, 0x02, 0x00];
        let iter = &mut op.iter().map(|x| *x);

        let (instr, mut operiter) = decode_instr(iter);
        assert_eq!(instr, InstructionType::BUGW);
        assert_eq!(operiter.next().unwrap(), (Ok(OperandMode::DataWord(2)), OperandWidth::Word));
    }

    #[test]
    fn decode_ret() {
        let op = vec![0x04];
        let iter = &mut op.iter().map(|x| *x);

        let (instr, mut operiter) = decode_instr(iter);
        assert_eq!(instr, InstructionType::RET);
        assert_eq!(operiter.next(), None);
    }
}