pub mod sub_record;

use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

use super::record_flags::RecordFlags;
use self::sub_record::*;
use binary::*;

esx_record!(SkilRecord, SkilSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  use esx::specialization::Specialization;

  read_write_test!(skil_record_read_write, SkilRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      SkilSubRecord::Indx(SkilIndx {
        skill_id: 42
      }),
      SkilSubRecord::Skdt(SkilSkdt {
        attribute: 42,
        specialization: Specialization::Combat,
        use_value: [42f32, 42f32, 42f32, 42f32]
      }),
      SkilSubRecord::Desc(SkilDesc {
        description: String::from("42")
      }),
    ],
  });
}
