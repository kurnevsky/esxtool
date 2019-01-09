use super::spell_school::SpellSchool;
use super::flags::Flags;
use crate::binary::*;

esx_sub_record_simple! {
  struct MgefMedt {
    spell_school: SpellSchool,
    base_cost: f32,
    flags: Flags,
    red: u32,
    blue: u32,
    green: u32,
    speed_x: f32,
    size_x: f32,
    size_cap: f32
  }
}
