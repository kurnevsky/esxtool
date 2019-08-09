use crate::binary::Binary;

esx_bitflags! {
  struct Flags: u32 {
    const SPELLMAKING = 0x0000_0200;
    const ENCHANTING = 0x0000_0400;
    const NEGATIVE = 0x0000_0800;
  }
}
