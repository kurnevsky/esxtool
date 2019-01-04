use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct RaceDesc(description)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(race_desc_read_write, RaceDesc {
    description: String::from("42")
  });
}
