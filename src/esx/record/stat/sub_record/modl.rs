use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct StatModl(model)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(stat_modl_read_write, StatModl {
    model: String::from("42")
  });
}
