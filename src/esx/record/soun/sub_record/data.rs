use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use binary::*;

esx_sub_record_simple! {
  struct SounData {
    volume: u8,
    min_range: u8,
    max_range: u8
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(soun_data_read_write, SounData {
    volume: 42,
    min_range: 42,
    max_range: 42,
  });
}
