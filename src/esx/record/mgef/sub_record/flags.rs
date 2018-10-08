use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use binary::Binary;

bitflags! {
  pub struct Flags: u32 {
    const Spellmaking = 0x0000_0200;
    const Enchanting = 0x0000_0400;
    const Negative = 0x0000_0800;
  }
}

bitflags_binary!(Flags);
