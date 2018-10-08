use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use binary::*;

esx_sub_record_simple! {
  struct MgefIndx {
    effect_id: u32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_indx_read_write, MgefIndx {
    effect_id: 42
  });
}
