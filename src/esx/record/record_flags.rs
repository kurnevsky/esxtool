use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use binary::Binary;

bitflags! {
  pub struct RecordFlags: u32 {
    const Unknown = 0x0000_0020;
    const Blocked = 0x0000_2000;
    const Persistent = 0x0000_0400;
  }
}

bitflags_binary!(RecordFlags);