pub mod sub_record;

use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(Tes3Record, Tes3SubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(tes3_record_read_write, Tes3Record {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      Tes3SubRecord::Hedr(Tes3Hedr {
        version: 42f32,
        file_type: FileType::Esp,
        company_name: String::from("42"),
        file_description: String::from("43"),
        num_records: 43
      }),
      Tes3SubRecord::Mast(Tes3Mast {
        master_file_name: String::from("42")
      }),
      Tes3SubRecord::Data(Tes3Data {
        master_size: 42
      }),
    ],
  });
}
