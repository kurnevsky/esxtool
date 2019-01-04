pub mod sub_record;

use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(GlobRecord, GlobSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(gmst_record_read_write, GlobRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      GlobSubRecord::Name(GlobName {
        name: String::from("42")
      }),
    ],
  });
}
