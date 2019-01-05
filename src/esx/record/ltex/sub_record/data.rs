use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct LtexData(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(ltex_data_read_write, LtexData {
    name: String::from("42")
  });
}
