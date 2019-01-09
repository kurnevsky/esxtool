use crate::binary::Binary;

esx_bitflags! {
  struct Flags: u32 {
    const Spellmaking = 0x0000_0200;
    const Enchanting = 0x0000_0400;
    const Negative = 0x0000_0800;
  }
}
