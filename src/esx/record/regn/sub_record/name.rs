use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct RegnName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(regn_name_read_write, RegnName {
    name: String::from("42")
  });
}
