use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use binary::*;

esx_sub_record_simple! {
  struct Tes3Data {
    master_size: u64
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(tes3_data_read_write, Tes3Data {
    master_size: 42
  });
}
