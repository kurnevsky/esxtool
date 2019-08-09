use crate::binary::Binary;

esx_bitflags! {
  struct Flags: u32 {
    const HIDDEN = 0x0000_0001;
  }
}
