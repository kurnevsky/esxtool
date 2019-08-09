use crate::binary::Binary;

esx_bitflags! {
  struct Flags: u32 {
    const MAGICAL = 0x0000_0001;
    const SILVER = 0x0000_0002;
  }
}
