#[macro_use]
mod esx_macro;
pub mod record;
pub mod util;
pub mod specialization;
pub mod color_ref;

use std::io::{Error, ErrorKind, Read, Result, Seek, Write};
use encoding::Encoding;

use self::record::Record;
use self::record::tes3::sub_record::Tes3SubRecord;
use crate::binary::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Esx {
  pub records: Vec<Record>
}

impl Binary for Esx {
  fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
    let first_record = Record::read(input, encoding)?;
    let num_records = if let Record::Tes3(ref tes3) = first_record {
      if let Some(Tes3SubRecord::Hedr(hedr)) = tes3.sub_records.first() {
        hedr.num_records
      } else {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid TES3 record"));
      }
    } else {
      return Err(Error::new(ErrorKind::InvalidData, "Invalid first record"));
    };
    let mut records = Vec::with_capacity(num_records as usize);
    records.push(first_record);
    for _ in 0 .. num_records {
      records.push(Record::read(input, encoding)?);
    }
    Ok(Esx {
      records
    })
  }

  fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
    let mut len = 0;
    for record in &self.records {
      len += record.write(output, encoding)?;
    }
    Ok(len)
  }
}
