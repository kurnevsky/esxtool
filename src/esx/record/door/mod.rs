pub mod sub_record;

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(DoorRecord, DoorSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(door_record_read_write, DoorRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      DoorSubRecord::Name(DoorName {
        name: String::from("42")
      }),
      DoorSubRecord::Fnam(DoorFnam {
        name: String::from("42")
      }),
      DoorSubRecord::Modl(DoorModl {
        model: String::from("42")
      }),
      DoorSubRecord::Scri(DoorScri {
        script: String::from("42")
      }),
      DoorSubRecord::Snam(DoorSnam {
        name: String::from("42")
      }),
      DoorSubRecord::Anam(DoorAnam {
        name: String::from("42")
      }),
    ],
  });
}
