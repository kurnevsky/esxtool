use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_data! {
  struct ColorRef {
    red: u8,
    green: u8,
    blue: u8,
    null: u8
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(color_ref_read_write, ColorRef {
    red: 42,
    green: 42,
    blue: 42,
    null: 42,
  });
}
