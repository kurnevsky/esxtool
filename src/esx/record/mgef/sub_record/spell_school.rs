use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use binary::*;

esx_enum! {
  enum SpellSchool: u32 {
    Alteration = 0,
    Conjuration = 1,
    Destruction = 2,
    Illusion = 3,
    Mysticism = 4,
    Restoration = 5
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(spell_school_alteration_read_write, SpellSchool::Alteration);
  read_write_test!(spell_school_conjuration_read_write, SpellSchool::Conjuration);
  read_write_test!(spell_school_destruction_read_write, SpellSchool::Destruction);
  read_write_test!(spell_school_illusion_read_write, SpellSchool::Illusion);
  read_write_test!(spell_school_mysticism_read_write, SpellSchool::Mysticism);
  read_write_test!(spell_school_restoration_read_write, SpellSchool::Restoration);
}
