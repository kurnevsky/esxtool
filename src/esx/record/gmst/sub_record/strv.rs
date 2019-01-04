use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct GmstStrv(value)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(gmst_strv_read_write, GmstStrv {
    value: String::from("42")
  });
}
