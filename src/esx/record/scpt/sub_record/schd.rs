use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use encoding::Encoding;

use binary::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ScptSchd {
  pub name: String,
  pub num_shorts: u32,
  pub num_longs: u32,
  pub num_floats: u32,
  pub script_data_size: u32,
  pub local_var_size: u32,
}

impl Binary for ScptSchd {
  fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
    let size = u32::read(input, encoding)?;
    if size != 52 {
      return Err(Error::new(ErrorKind::InvalidData, "Wrong size of ScptSchd"));
    }
    let name = read_string(input, encoding, 32)?;
    let num_shorts = u32::read(input, encoding)?;
    let num_longs = u32::read(input, encoding)?;
    let num_floats = u32::read(input, encoding)?;
    let script_data_size = u32::read(input, encoding)?;
    let local_var_size = u32::read(input, encoding)?;
    Ok(ScptSchd {
      name,
      num_shorts,
      num_longs,
      num_floats,
      script_data_size,
      local_var_size,
    })
  }

  fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
    52u32.write(output, encoding)?;
    write_string_exact(output, encoding, &self.name, 32)?;
    self.num_shorts.write(output, encoding)?;
    self.num_longs.write(output, encoding)?;
    self.num_floats.write(output, encoding)?;
    self.script_data_size.write(output, encoding)?;
    self.local_var_size.write(output, encoding)?;
    Ok(56)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(scpt_schd_read_write, ScptSchd {
    name: String::from("42"),
    num_shorts: 42,
    num_longs: 42,
    num_floats: 42,
    script_data_size: 42,
    local_var_size: 42,
  });
}
