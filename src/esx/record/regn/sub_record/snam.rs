use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use encoding::Encoding;

use binary::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RegnSnam {
  pub sound_name: String,
  pub chance: u8,
}

impl Binary for RegnSnam {
  fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
    let size = u32::read(input, encoding)?;
    if size != 33 {
      return Err(Error::new(ErrorKind::InvalidData, "Wrong size of RegnSnam"));
    }
    let sound_name = read_string(input, encoding, 32)?;
    let chance = u8::read(input, encoding)?;
    Ok(RegnSnam {
      sound_name,
      chance,
    })
  }

  fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
    33u32.write(output, encoding)?;
    write_string_exact(output, encoding, &self.sound_name, 32)?;
    self.chance.write(output, encoding)?;
    Ok(37)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(regn_snam_read_write, RegnSnam {
    sound_name: String::from("42"),
    chance: 42,
  });
}
