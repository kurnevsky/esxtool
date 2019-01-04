use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use encoding::{DecoderTrap, EncoderTrap, Encoding};

use crate::binary::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ScptScvr {
  pub variables: Vec<String>,
}

impl Binary for ScptScvr {
  fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
    let size = u32::read(input, encoding)?;
    let mut buffer = vec![0; size as usize];
    input.read_exact(&mut buffer)?;
    let mut variables = Vec::new();
    for variable in buffer.split(|&b| b == 0).filter(|variable| !variable.is_empty()) {
      variables.push(encoding.decode(&variable, DecoderTrap::Strict).map_err(|e|
        Error::new(ErrorKind::InvalidData, e.to_string())
      )?);
    }
    Ok(ScptScvr {
      variables
    })
  }

  fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
    let mut buf = Vec::new();
    for variable in &self.variables {
      encoding.encode_to(variable, EncoderTrap::Strict, &mut buf).map_err(|e|
        Error::new(ErrorKind::InvalidData, e.to_string())
      )?;
      buf.push(0);
    }
    let len = buf.len() as u32;
    len.write(output, encoding)?;
    output.write_all(&buf)?;
    Ok(len + 4)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(scpt_scvr_read_write, ScptScvr {
    variables: vec![
      String::from("42"),
      String::from("43"),
    ],
  });
}
