use crate::binary::Binary;

bitflags! {
  pub struct Flags: u32 {
    const Hidden = 0x0000_0001;
  }
}

bitflags_binary!(Flags);
