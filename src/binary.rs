use std::io::{Error, ErrorKind, Read, Result, Seek, Write};
use std::mem::{size_of, uninitialized};

use byteorder::LE;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use crate::primitive::{PrimitiveRead, PrimitiveWrite};

pub trait Binary : Sized {
  fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self>;
  fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32>;
}

impl<T: PrimitiveRead + PrimitiveWrite + Copy> Binary for T {
  fn read<R: Read + Seek, E: Encoding>(input: &mut R, _encoding: &E) -> Result<Self> {
    Self::primitive_read::<R, LE>(input)
  }

  fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, _encoding: &E) -> Result<u32> {
    self.primitive_write::<W, LE>(output)?;
    Ok(size_of::<Self>() as u32)
  }
}

macro_rules! binary_array {
  ( $( $n:expr )* ) => {
    $(
      impl<T: Binary> Binary for [T; $n] {
        fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
          let mut res: [T; $n] = unsafe { uninitialized() };
          for item in &mut res {
              *item = T::read(input, encoding)?;
          }
          Ok(res)
        }

        fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
          let mut len = 0;
          for e in self {
            len += e.write(output, encoding)?;
          }
          Ok(len)
        }
      }
    )*
  }
}

binary_array!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15);

pub fn read_string<R: Read, E: Encoding>(input: &mut R, encoding: &E, len: u32) -> Result<String> {
  let mut buf = vec![0; len as usize];
  input.read_exact(&mut buf)?;
  let len_not_null = buf.iter().rposition(|&c| c != 0).map_or(0, |idx| idx + 1);
  encoding.decode(&buf[0..len_not_null], DecoderTrap::Strict).map_err(|e|
    Error::new(ErrorKind::InvalidData, e.to_string())
  )
}

pub fn write_string_exact<W: Write, E: Encoding>(output: &mut W, encoding: &E, s: &str, len: u32) -> Result<u32> {
  let mut buf = Vec::with_capacity(len as usize);
  encoding.encode_to(s, EncoderTrap::Strict, &mut buf).map_err(|e|
    Error::new(ErrorKind::InvalidData, e.to_string())
  )?;
  if buf.len() > len as usize {
    return Err(Error::new(ErrorKind::InvalidData, "Too long string"));
  }
  buf.resize(len as usize, 0);
  output.write_all(&buf)?;
  Ok(len)
}

pub fn read_name<R: Read>(input: &mut R) -> Result<[u8; 4]> {
  let mut buf = [0; 4];
  input.read_exact(&mut buf)?;
  Ok(buf)
}

#[cfg(test)]
macro_rules! read_write_test (
  ($type:ty) => (
    #[test]
    fn read_write() {
      use encoding::all::ASCII;
      use std::io::Cursor;
      use crate::samples::Samples;
      let mut buf = Cursor::new(Vec::new());
      for before in <$type>::samples() {
          let len = before.write(&mut buf, ASCII).unwrap();
          assert_eq!(len, buf.get_ref().len() as u32);
          buf.set_position(0);
          let after = Binary::read(&mut buf, ASCII).unwrap();
          assert_eq!(before, after);
          buf.set_position(0);
          buf.get_mut().clear();
      }
    }
  )
);
