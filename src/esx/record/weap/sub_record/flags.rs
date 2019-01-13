use crate::binary::Binary;

esx_bitflags! {
  struct Flags: u32 {
    const Magical = 0x0000_0001;
    const Silver = 0x0000_0002;
  }
}
