pub mod sub_record;

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(SounRecord, SounSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(soun_record_read_write, SounRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      SounSubRecord::Name(SounName {
        name: String::from("42")
      }),
      SounSubRecord::Fnam(SounFnam {
        name: String::from("42")
      }),
      SounSubRecord::Data(SounData {
        volume: 42,
        min_range: 42,
        max_range: 42,
      }),
    ],
  });
}
