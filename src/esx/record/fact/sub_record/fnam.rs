use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct FactFnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(fact_fnam_read_write, FactFnam {
    name: String::from("42")
  });
}
