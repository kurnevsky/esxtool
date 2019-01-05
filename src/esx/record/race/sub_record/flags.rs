use crate::binary::Binary;

bitflags! {
  pub struct Flags: u32 {
    const Playable = 0x0000_0001;
    const BeastRace = 0x0000_0002;
  }
}

bitflags_binary!(Flags);
