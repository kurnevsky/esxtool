use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct SkilDesc(description)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(skil_desc_read_write, SkilDesc {
    description: String::from("42")
  });
}
