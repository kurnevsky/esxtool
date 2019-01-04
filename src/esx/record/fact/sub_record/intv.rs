use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_simple! {
  struct FactIntv {
    value: i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(fact_intv_read_write, FactIntv {
    value: 42
  });
}
