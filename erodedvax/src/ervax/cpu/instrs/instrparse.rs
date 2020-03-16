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
    field_id: u8,
    fm: &'static [FieldMode],
    fw: &'static [OperandWidth],
    bytes: &'a mut I,
}

impl<'a, I: Iterator> OperandIter<'a, I> 
    where I: Iterator<Item = u8>
{
    #[inline]
    pub fn from_instr<'b>(inst: InstructionType, bytes: &'b mut I) -> OperandIter<'b, I> {
        OperandIter {
            field_id: 0,
            fm: inst.field_modes(),
            fw: inst.field_widths(),
            bytes,
        }
    }

    pub(crate) fn from_raw<'b>(fm: &'static [FieldMode], fw: &'static [OperandWidth], bytes: &'b mut I) -> OperandIter<'b, I> {
        OperandIter {
            field_id: 0,
            fm,
            fw,
            bytes,
        }
    }

    /// Consumes the OperandIter and returns the bytes iter it was created with, and the field it was on.
    #[inline]
    pub fn destructure(self) -> (&'a mut I, u8) {
        return (self.bytes, self.field_id);
    }

    #[inline(always)]
    pub fn is_done(&self) -> bool {
        self.field_id == 255
    }

    #[inline(always)]
    pub fn set_done(&mut self) {
        self.field_id = 255;
    }
}

impl<'a, I: Iterator> Iterator for OperandIter<'a, I>
    where I: Iterator<Item = u8>
{
    type Item = Result<OperandMode, OperandParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let curfield = self.field_id as usize;

        if self.is_done() {
            return None;
        }

        if curfield >= self.fm.len() {
            self.set_done();
            return None;
        }

        let curfm = unsafe { *self.fm.get_unchecked(curfield) };
        let curfw = unsafe { *self.fw.get_unchecked(curfield) };

        self.field_id += 1;

        match curfm {
            FieldMode::Data => {
                match curfw {
                    OperandWidth::Byte => {
                        if let Some(next) = self.bytes.next() {
                            return Some(Ok(OperandMode::DataByte(next)));
                        }
                        return Some(Err(OperandParseError::OutOfBytes));
                    }
                    OperandWidth::Word => {
                        match get_u16_from_stream(self.bytes) {
                            Some(v) => return Some(Ok(OperandMode::DataWord(v))),
                            None => return Some(Err(OperandParseError::OutOfBytes)),
                        }
                    }
                    OperandWidth::Longword => {
                        match get_u32_from_stream(self.bytes) {
                            Some(v) => return Some(Ok(OperandMode::DataLong(v))),
                            None => return Some(Err(OperandParseError::OutOfBytes)),
                        }
                    }
                    _ => unimplemented!("Quadword and Octaword data width was not needed at time of implementation")
                }
            },
            FieldMode::VariableLengthTable => {
                return None;
            },
            v => {
                let opres = OperandMode::read_operand(self.bytes, curfw, true);

                if let Ok(om) = opres 
                {
                    if om.is_valid_in_fieldmode(v) {
                        return Some(Ok(om));
                    } else {
                        return Some(Err(OperandParseError::InvalidMode));
                    }
                } else {
                    return Some(opres);
                }
            }
        }
    }
}

pub fn decode_instr<'a, I>(bytes: &'a mut I) -> Option<(InstructionType, OperandIter<'a, I>)>
    where I: Iterator<Item = u8>
{
    if let Some(instr) = InstructionType::from_instrid(bytes) {
        Some((instr, OperandIter::from_instr(instr, bytes)))
    } else {
        None
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
    fn decode_addb2_imm_reg() {
        let op = vec![0x80, 0x8F, 0x02, 0x51];
        let iter = &mut op.iter().map(|x| *x);

        let (instr, mut operiter) = decode_instr(iter).unwrap();
        assert_eq!(instr, InstructionType::ADDB2);
        assert_eq!(operiter.next().unwrap(), Ok(OperandMode::Immediate8(2)));
        assert_eq!(operiter.next().unwrap(), Ok(OperandMode::Register(RegID(1))));
    }

    #[test]
    fn decode_bugw() {
        let op = vec![0xFF, 0xFE, 0x02, 0x00];
        let iter = &mut op.iter().map(|x| *x);

        let (instr, mut operiter) = decode_instr(iter).unwrap();
        assert_eq!(instr, InstructionType::BUGW);
        assert_eq!(operiter.next().unwrap(), Ok(OperandMode::DataWord(2)));
    }

    #[test]
    fn decode_ret() {
        let op = vec![0x04];
        let iter = &mut op.iter().map(|x| *x);

        let (instr, mut operiter) = decode_instr(iter).unwrap();
        assert_eq!(instr, InstructionType::RET);
        assert_eq!(operiter.next(), None);
    }

    #[test]
    fn decode_invalid() {
        let op = vec![0xFF, 0xFF];
        let iter = &mut op.iter().map(|x| *x);

        if let Some(_) = decode_instr(iter) {
            panic!("Decoded invalid successfully???");
        } else {

        }
    }
}