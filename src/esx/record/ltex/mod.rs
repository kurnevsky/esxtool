pub mod sub_record;

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(LtexRecord, LtexSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(ltex_record_read_write, LtexRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      LtexSubRecord::Name(LtexName {
        name: String::from("42")
      }),
      LtexSubRecord::Intv(LtexIntv {
        value: 42
      }),
      LtexSubRecord::Data(LtexData {
        name: String::from("42")
      }),
    ],
  });
}
