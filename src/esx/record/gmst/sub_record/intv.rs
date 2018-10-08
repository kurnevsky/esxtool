use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use binary::*;

esx_sub_record_simple! {
  struct GmstIntv {
    value: i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(gmst_intv_read_write, GmstIntv {
    value: 42
  });
}
