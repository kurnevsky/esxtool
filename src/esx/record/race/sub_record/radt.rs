use super::skill_bonus::SkillBonus;
use super::flags::Flags;
use crate::binary::*;

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
