pub mod sub_record;

use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(BsgnRecord, BsgnSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(bsgn_record_read_write, BsgnRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      BsgnSubRecord::Name(BsgnName {
        name: String::from("42")
      }),
      BsgnSubRecord::Fnam(BsgnFnam {
        name: String::from("42")
      }),
      BsgnSubRecord::Tnam(BsgnTnam {
        name: String::from("42")
      }),
      BsgnSubRecord::Desc(BsgnDesc {
        description: String::from("42")
      }),
      BsgnSubRecord::Npcs(BsgnNpcs {
        ability: String::from("42"),
      }),
    ],
  });
}
