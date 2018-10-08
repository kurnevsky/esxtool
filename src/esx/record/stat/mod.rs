pub mod sub_record;

use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

use super::record_flags::RecordFlags;
use self::sub_record::*;
use binary::*;

esx_record!(StatRecord, StatSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(stat_record_read_write, StatRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      StatSubRecord::Name(StatName {
        name: String::from("42")
      }),
      StatSubRecord::Modl(StatModl {
        model: String::from("42")
      }),
    ],
  });
}
