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

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_medt_read_write, MgefMedt {
    spell_school: SpellSchool::Alteration,
    base_cost: 42f32,
    flags: Flags::Spellmaking,
    red: 42,
    blue: 42,
    green: 42,
    speed_x: 42f32,
    size_x: 42f32,
    size_cap: 42f32,
  });
}
