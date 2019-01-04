use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_simple! {
  struct SkilIndx {
    skill_id: u32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(skil_indx_read_write, SkilIndx {
    skill_id: 42
  });
}
