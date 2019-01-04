use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct RaceName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(race_name_read_write, RaceName {
    name: String::from("42")
  });
}
