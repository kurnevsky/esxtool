use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct BsgnName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(bsgn_name_read_write, BsgnName {
    name: String::from("42")
  });
}