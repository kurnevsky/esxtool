use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use encoding::Encoding;

use crate::binary::*;
use crate::samples::Samples;

#[derive(Debug, Clone, PartialEq)]
pub struct BsgnNpcs { //TODO: macro?
  pub ability: String,
}

impl Binary for BsgnNpcs {
  fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
    let size = u32::read(input, encoding)?;
    if size != 32 {
      return Err(Error::new(ErrorKind::InvalidData, "Wrong size of BsgnNpcs"));
    }
    let ability = read_string(input, encoding, 32)?;
    Ok(BsgnNpcs {
      ability,
    })
  }

  fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
    32u32.write(output, encoding)?;
    write_string_exact(output, encoding, &self.ability, 32)?;
    Ok(36)
  }
}

#[cfg(test)]
impl Samples for BsgnNpcs {
  fn single() -> Self {
    BsgnNpcs {
      ability: String::from("42"),
    }
  }
}

read_write_test!(BsgnNpcs);
