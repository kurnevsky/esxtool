use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct BsgnTnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(bsgn_tnam_read_write, BsgnTnam {
    name: String::from("42")
  });
}
