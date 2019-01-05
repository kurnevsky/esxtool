use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct StatName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(stat_name_read_write, StatName {
    name: String::from("42")
  });
}
