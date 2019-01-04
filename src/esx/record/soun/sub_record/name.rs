use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct SounName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(soun_name_read_write, SounName {
    name: String::from("42")
  });
}
