use std::io;

use super::Numeric;

pub trait BitWrite {
    fn write<U>(&mut self, bits: u32, value: U) -> Result<(), io::Error>
        where U: Numeric;

    fn write_signed(&mut self, bits: u32, value: i32) -> Result<(), io::Error>;

    fn write_bytes(&mut self, buf: &[u8]) -> Result<(), io::Error>;

    fn write_unary0(&mut self, value: u32) -> Result<(), io::Error>;

    fn write_unary1(&mut self, value: u32) -> Result<(), io::Error>;

    fn byte_aligned(&self) -> bool;

    fn byte_align(&mut self) -> Result<(), io::Error>;
}

pub struct BitWriterBE<'a> {
    writer: &'a mut io::Write,
    buffer: [u8; 1],
    bits: usize
}

impl<'a> BitWriterBE<'a> {
    pub fn new(writer: &mut io::Write) -> BitWriterBE {
        BitWriterBE{writer: writer, buffer: [0], bits: 0}
    }

    fn write_bit(&mut self, bit: bool) -> Result<(), io::Error> {
        if bit {
            self.buffer[0] |= 1 << (7 - self.bits);
        }
        self.bits += 1;
        if self.bits == 8 {
            self.writer.write_all(&self.buffer)?;
            self.buffer[0] = 0;
            self.bits = 0;
        }
        Ok(())
    }
}

impl<'a> BitWrite for BitWriterBE<'a> {
    fn write<U>(&mut self, mut bits: u32, value: U) -> Result<(), io::Error>
        where U: Numeric {
        /*FIXME - generalize this to any sort of unsigned value*/
        while bits > 0 {
            let mask = U::one() << (bits - 1);
            self.write_bit((value & mask).to_bit())?;
            bits -= 1;
        }
        Ok(())
    }

    fn write_signed(&mut self, bits: u32, value: i32) -> Result<(), io::Error> {
        /*FIXME - generalize this to any sort of signed value*/
        if value >= 0 {
            self.write(1, 0u8)?;
            self.write(bits - 1, value as u32)
        } else {
            self.write(1, 1u8)?;
            self.write(bits - 1, ((1 << (bits - 1)) + value) as u32)
        }
    }

    fn write_bytes(&mut self, buf: &[u8]) -> Result<(), io::Error> {
        if self.byte_aligned() {
            self.writer.write_all(buf)
        } else {
            for b in buf {
                self.write(8, *b)?;
            }
            Ok(())
        }
    }

    fn write_unary0(&mut self, value: u32) -> Result<(), io::Error> {
        /*FIXME - optimize this*/
        for _ in 0..value {
            self.write(1, 1u8)?;
        }
        self.write(1, 0u8)
    }

    fn write_unary1(&mut self, value: u32) -> Result<(), io::Error> {
        /*FIXME - optimize this*/
        for _ in 0..value {
            self.write(1, 0u8)?;
        }
        self.write(1, 1u8)
    }

    fn byte_aligned(&self) -> bool {
        self.bits == 0
    }

    fn byte_align(&mut self) -> Result<(), io::Error> {
        /*FIXME - optimize this*/
        while !self.byte_aligned() {
            self.write(1, 0u8)?;
        }
        Ok(())
    }
}

pub struct BitWriterLE<'a> {
    writer: &'a mut io::Write,
    buffer: [u8; 1],
    bits: usize
}

impl<'a> BitWriterLE<'a> {
    pub fn new(writer: &mut io::Write) -> BitWriterLE {
        BitWriterLE{writer: writer, buffer: [0], bits: 0}
    }

    fn write_bit(&mut self, bit: bool) -> Result<(), io::Error> {
        if bit {
            self.buffer[0] |= 1 << self.bits;
        }
        self.bits += 1;
        if self.bits == 8 {
            self.writer.write_all(&self.buffer)?;
            self.buffer[0] = 0;
            self.bits = 0;
        }
        Ok(())
    }
}

impl<'a> BitWrite for BitWriterLE<'a> {
    fn write<U>(&mut self, mut bits: u32, mut value: U) -> Result<(), io::Error>
        where U: Numeric {
        while bits > 0 {
            self.write_bit((value & U::one()).to_bit())?;
            value >>= U::one();
            bits -= 1;
        }
        Ok(())
    }

    fn write_signed(&mut self, bits: u32, value: i32) -> Result<(), io::Error> {
        /*FIXME - generalize this to any sort of signed value*/
        if value >= 0 {
            self.write(bits - 1, value as u32)?;
            self.write(1, 0u8)
        } else {
            self.write(bits - 1, ((1 << (bits - 1)) + value) as u32)?;
            self.write(1, 1u8)
        }
    }

    fn write_bytes(&mut self, buf: &[u8]) -> Result<(), io::Error> {
        if self.byte_aligned() {
            self.writer.write_all(buf)
        } else {
            for b in buf {
                self.write(8, *b)?;
            }
            Ok(())
        }
    }

    fn write_unary0(&mut self, value: u32) -> Result<(), io::Error> {
        /*FIXME - optimize this*/
        for _ in 0..value {
            self.write(1, 1u8)?;
        }
        self.write(1, 0u8)
    }

    fn write_unary1(&mut self, value: u32) -> Result<(), io::Error> {
        /*FIXME - optimize this*/
        for _ in 0..value {
            self.write(1, 0u8)?;
        }
        self.write(1, 1u8)
    }

    fn byte_aligned(&self) -> bool {
        self.bits == 0
    }

    fn byte_align(&mut self) -> Result<(), io::Error> {
        /*FIXME - optimize this*/
        while !self.byte_aligned() {
            self.write(1, 0u8)?;
        }
        Ok(())
    }
}
