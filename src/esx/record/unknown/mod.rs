use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

use encoding::Encoding;

use super::record_flags::RecordFlags;
use crate::binary::*;

#[derive(Debug, Clone, PartialEq)]
pub struct UnknownRecord {
  pub name: [u8; 4],
  pub unknown: u32,
  pub flags: RecordFlags,
  pub sub_records: Vec<UnknownSubRecord>,
}

impl Binary for UnknownRecord {
  fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
    let name = read_name(input)?;
    let size = u32::read(input, encoding)?;
    let unknown = u32::read(input, encoding)?;
    let flags = RecordFlags::read(input, encoding)?;
    let pos = input.seek(SeekFrom::Current(0))?;
    let mut cur_pos = pos;
    let mut sub_records = Vec::new();
    while size > (cur_pos - pos) as u32 {
      sub_records.push(UnknownSubRecord::read(input, encoding)?);
      cur_pos = input.seek(SeekFrom::Current(0))?;
    }
    if size != (cur_pos - pos) as u32 {
      return Err(Error::new(ErrorKind::InvalidData, "Wrong length of record"));
    }
    Ok(UnknownRecord {
      name,
      unknown,
      flags,
      sub_records,
    })
  }

  fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
    output.write_all(&self.name)?;
    let pos = output.seek(SeekFrom::Current(4))?;
    self.unknown.write(output, encoding)?;
    self.flags.write(output, encoding)?;
    let mut len = 0;
    for sub_record in &self.sub_records {
      len += sub_record.write(output, encoding)?;
    }
    output.seek(SeekFrom::Start(pos - 4))?;
    len.write(output, encoding)?;
    output.seek(SeekFrom::End(0))?;
    Ok(len + 16)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnknownSubRecord {
  pub name: [u8; 4],
  pub data: Vec<u8>
}

impl Binary for UnknownSubRecord {
  fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
    let name = read_name(input)?;
    let size = u32::read(input, encoding)?;
    let mut data = vec![0; size as usize];
    input.read_exact(&mut data)?;
    Ok(UnknownSubRecord {
      name,
      data,
    })
  }

  fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
    output.write_all(&self.name)?;
    (self.data.len() as u32).write(output, encoding)?;
    output.write_all(&self.data)?;
    Ok(self.data.len() as u32 + 8)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(unknown_record_read_write, UnknownRecord {
    name: *b"ABCD",
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      UnknownSubRecord {
        name: *b"ABCD",
        data: vec![42; 123],
      }
    ],
  });

  read_write_test!(unknown_sub_record_read_write, UnknownSubRecord {
    name: *b"ABCD",
    data: vec![42; 123],
  });
}
