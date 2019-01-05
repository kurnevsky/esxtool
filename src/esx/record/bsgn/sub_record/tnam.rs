use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct BsgnTnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(bsgn_tnam_read_write, BsgnTnam {
    name: String::from("42")
  });
}
