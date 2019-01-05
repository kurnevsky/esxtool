use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct RegnFnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(regn_fnam_read_write, RegnFnam {
    name: String::from("42")
  });
}
