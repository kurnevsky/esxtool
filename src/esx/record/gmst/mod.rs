pub mod sub_record;

use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(GmstRecord, GmstSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(gmst_record_read_write, GmstRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      GmstSubRecord::Name(GmstName {
        name: String::from("42")
      }),
      GmstSubRecord::Strv(GmstStrv {
        value: String::from("42")
      }),
      GmstSubRecord::Intv(GmstIntv {
        value: 42
      }),
      GmstSubRecord::Fltv(GmstFltv {
        value: 42f32
      }),
    ],
  });
}
