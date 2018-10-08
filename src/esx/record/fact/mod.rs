pub mod sub_record;

use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

use super::record_flags::RecordFlags;
use self::sub_record::*;
use binary::*;

esx_record!(FactRecord, FactSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(fact_record_read_write, FactRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      FactSubRecord::Name(FactName {
        name: String::from("42")
      }),
      FactSubRecord::Fnam(FactFnam {
        name: String::from("42")
      }),
      FactSubRecord::Fadt(FactFadt {
        attribute_id_1: 42,
        attribute_id_2: 42,
        rank_data: [
          RankData {
            attribute_1: 42,
            attribute_2: 42,
            first_skill: 42,
            second_skill: 42,
            faction: 42
          },
          RankData {
            attribute_1: 42,
            attribute_2: 42,
            first_skill: 42,
            second_skill: 42,
            faction: 42
          },
          RankData {
            attribute_1: 42,
            attribute_2: 42,
            first_skill: 42,
            second_skill: 42,
            faction: 42
          },
          RankData {
            attribute_1: 42,
            attribute_2: 42,
            first_skill: 42,
            second_skill: 42,
            faction: 42
          },
          RankData {
            attribute_1: 42,
            attribute_2: 42,
            first_skill: 42,
            second_skill: 42,
            faction: 42
          },
          RankData {
            attribute_1: 42,
            attribute_2: 42,
            first_skill: 42,
            second_skill: 42,
            faction: 42
          },
          RankData {
            attribute_1: 42,
            attribute_2: 42,
            first_skill: 42,
            second_skill: 42,
            faction: 42
          },
          RankData {
            attribute_1: 42,
            attribute_2: 42,
            first_skill: 42,
            second_skill: 42,
            faction: 42
          },
          RankData {
            attribute_1: 42,
            attribute_2: 42,
            first_skill: 42,
            second_skill: 42,
            faction: 42
          },
          RankData {
            attribute_1: 42,
            attribute_2: 42,
            first_skill: 42,
            second_skill: 42,
            faction: 42
          },
        ],
        skill_id: [42, 42, 42, 42, 42, 42, 42],
        flags: Flags::Hidden,
      }),
      FactSubRecord::Rnam(FactRnam {
        name: String::from("42")
      }),
      FactSubRecord::Intv(FactIntv {
        value: 42
      }),
    ],
  });
}
