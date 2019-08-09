use crate::binary::Binary;

esx_bitflags! {
  struct RecordFlags: u32 {
    const UNKNOWN = 0x0000_0020;
    const PERSISTENT = 0x0000_0400;
    const BLOCKED = 0x0000_2000;
  }
}
