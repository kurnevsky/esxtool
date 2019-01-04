use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct FactAnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(fact_anam_read_write, FactAnam {
    name: String::from("42")
  });
}
