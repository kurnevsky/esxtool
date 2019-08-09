use crate::binary::Binary;

esx_bitflags! {
  struct Flags: u32 {
    const PLAYABLE = 0x0000_0001;
    const BEASTRACE = 0x0000_0002;
  }
}
