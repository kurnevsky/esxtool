use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_fixed_string! {
  struct RaceNpcs(name: 32)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(race_npcs_read_write, RaceNpcs {
    name: String::from("42")
  });
}
