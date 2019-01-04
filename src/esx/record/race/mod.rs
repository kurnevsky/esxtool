pub mod sub_record;

use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(RaceRecord, RaceSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(race_record_read_write, RaceRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      RaceSubRecord::Name(RaceName {
        name: String::from("42")
      }),
      RaceSubRecord::Fnam(RaceFnam {
        name: String::from("42")
      }),
      RaceSubRecord::Radt(RaceRadt {
        skill_bonuses: [
          SkillBonus {
            skill_id: 42,
            bonus: 42,
          },
          SkillBonus {
            skill_id: 42,
            bonus: 42,
          },
          SkillBonus {
            skill_id: 42,
            bonus: 42,
          },
          SkillBonus {
            skill_id: 42,
            bonus: 42,
          },
          SkillBonus {
            skill_id: 42,
            bonus: 42,
          },
          SkillBonus {
            skill_id: 42,
            bonus: 42,
          },
          SkillBonus {
            skill_id: 42,
            bonus: 42,
          },
        ],
        strength: [42, 42],
        intelligence: [42, 42],
        willpower: [42, 42],
        agility: [42, 42],
        speed: [42, 42],
        endurance: [42, 42],
        personality: [42, 42],
        luck: [42, 42],
        height: [42f32, 42f32],
        weight: [42f32, 42f32],
        flags: Flags::Playable,
      }),
      RaceSubRecord::Npcs(RaceNpcs {
        name: String::from("42")
      }),
      RaceSubRecord::Desc(RaceDesc {
        description: String::from("42")
      }),
    ],
  });
}
