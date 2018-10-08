use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct LtexData(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(ltex_data_read_write, LtexData {
    name: String::from("42")
  });
}
