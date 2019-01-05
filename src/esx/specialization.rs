use crate::binary::*;

esx_enum! {
  enum Specialization: u32 {
    Combat = 0,
    Magic = 1,
    Stealth = 2
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(specialization_combat_read_write, Specialization::Combat);
  read_write_test!(specialization_magic_read_write, Specialization::Magic);
  read_write_test!(specialization_stealth_read_write, Specialization::Stealth);
}
