mod skill_bonus;
mod flags;
mod name;
mod fnam;
mod radt;
mod npcs;
mod desc;

pub use self::skill_bonus::*;
pub use self::flags::*;
pub use self::name::*;
pub use self::fnam::*;
pub use self::radt::*;
pub use self::npcs::*;
pub use self::desc::*;

use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum RaceSubRecord {
    Name(RaceName) => b"NAME",
    Fnam(RaceFnam) => b"FNAM",
    Radt(RaceRadt) => b"RADT",
    Npcs(RaceNpcs) => b"NPCS",
    Desc(RaceDesc) => b"DESC"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(race_sub_record_name_read_write, RaceSubRecord::Name(RaceName {
    name: String::from("42")
  }));

  read_write_test!(race_sub_record_fnam_read_write, RaceSubRecord::Fnam(RaceFnam {
    name: String::from("42")
  }));

  read_write_test!(race_sub_record_radt_read_write, RaceSubRecord::Radt(RaceRadt {
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
  }));

  read_write_test!(race_sub_record_npcs_read_write, RaceSubRecord::Npcs(RaceNpcs {
    name: String::from("42")
  }));

  read_write_test!(race_sub_record_desc_read_write, RaceSubRecord::Desc(RaceDesc {
    description: String::from("42")
  }));
}
