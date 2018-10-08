use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct LtexName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(ltex_name_read_write, LtexName {
    name: String::from("42")
  });
}
