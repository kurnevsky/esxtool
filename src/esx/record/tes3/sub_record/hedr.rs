use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use encoding::Encoding;

use super::file_type::FileType;
use crate::binary::*;
use crate::samples::Samples;

#[derive(Debug, Clone, PartialEq)]
pub struct Tes3Hedr {
  pub version: f32,
  pub file_type: FileType,
  pub company_name: String,
  pub file_description: String,
  pub num_records: u32
}

impl Binary for Tes3Hedr {
  fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
    let size = u32::read(input, encoding)?;
    if size != 300 {
      return Err(Error::new(ErrorKind::InvalidData, "Wrong size of Tes3Hedr"));
    }
    let version = f32::read(input, encoding)?;
    let file_type = FileType::read(input, encoding)?;
    let company_name = read_string(input, encoding, 32)?;
    let file_description = read_string(input, encoding, 256)?;
    let num_records = u32::read(input, encoding)?;
    Ok(Tes3Hedr {
      version,
      file_type,
      company_name,
      file_description,
      num_records,
    })
  }

  fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
    300u32.write(output, encoding)?;
    self.version.write(output, encoding)?;
    self.file_type.write(output, encoding)?;
    write_string_exact(output, encoding, &self.company_name, 32)?;
    write_string_exact(output, encoding, &self.file_description, 256)?;
    self.num_records.write(output, encoding)?;
    Ok(304)
  }
}

#[cfg(test)]
impl Samples for Tes3Hedr {
  fn single() -> Self {
    Tes3Hedr {
      version: 42f32,
      file_type: FileType::Esp,
      company_name: String::from("42"),
      file_description: String::from("43"),
      num_records: 43
    }
  }
}

read_write_test!(Tes3Hedr);
