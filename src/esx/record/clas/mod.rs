pub mod sub_record;

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(ClasRecord, ClasSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  use crate::esx::specialization::Specialization;

  read_write_test!(clas_record_read_write, ClasRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      ClasSubRecord::Name(ClasName {
        name: String::from("42")
      }),
      ClasSubRecord::Fnam(ClasFnam {
        name: String::from("42")
      }),
      ClasSubRecord::Cldt(ClasCldt {
        attribute_id_1: 42,
        attribute_id_2: 42,
        specialization: Specialization::Combat,
        minor_id_1: 42,
        major_id_1: 42,
        minor_id_2: 42,
        major_id_2: 42,
        minor_id_3: 42,
        major_id_3: 42,
        minor_id_4: 42,
        major_id_4: 42,
        minor_id_5: 42,
        major_id_5: 42,
        flags: Flags::Playable,
        auto_calc_flags: AutoCalcFlags::Weapon,
      }),
      ClasSubRecord::Desc(ClasDesc {
        description: String::from("42")
      }),
    ],
  });
}
