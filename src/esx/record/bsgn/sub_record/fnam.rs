use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct BsgnFnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(bsgn_fnam_read_write, BsgnFnam {
    name: String::from("42")
  });
}
