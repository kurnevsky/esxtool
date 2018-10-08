use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct RegnFnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(regn_fnam_read_write, RegnFnam {
    name: String::from("42")
  });
}
