use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct StatName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(stat_name_read_write, StatName {
    name: String::from("42")
  });
}
