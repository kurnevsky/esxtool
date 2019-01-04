use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_data! {
  struct SkillBonus {
    skill_id: u32,
    bonus: u32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(skill_bonus_read_write, SkillBonus {
    skill_id: 42,
    bonus: 42,
  });
}
