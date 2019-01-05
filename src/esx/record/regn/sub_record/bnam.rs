use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct RegnBnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(regn_bnam_read_write, RegnBnam {
    name: String::from("42")
  });
}
