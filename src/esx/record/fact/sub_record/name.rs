use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct FactName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(fact_name_read_write, FactName {
    name: String::from("42")
  });
}
