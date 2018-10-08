use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct BsgnDesc(description)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(bsgn_desc_read_write, BsgnDesc {
    description: String::from("42")
  });
}
