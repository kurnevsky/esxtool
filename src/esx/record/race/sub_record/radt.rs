use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use super::skill_bonus::SkillBonus;
use super::flags::Flags;
use binary::*;

esx_sub_record_simple! {
  struct RaceRadt {
    skill_bonuses: [SkillBonus; 7],
    strength: [u32; 2],
    intelligence: [u32; 2],
    willpower: [u32; 2],
    agility: [u32; 2],
    speed: [u32; 2],
    endurance: [u32; 2],
    personality: [u32; 2],
    luck: [u32; 2],
    height: [f32; 2],
    weight: [f32; 2],
    flags: Flags
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(race_radt_read_write, RaceRadt {
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
  });
}
