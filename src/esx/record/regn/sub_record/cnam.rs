use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;
use crate::esx::color_ref::ColorRef;

esx_sub_record_simple! {
  struct RegnCnam {
    color_ref: ColorRef
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(regn_weat_read_write, RegnCnam {
    color_ref: ColorRef {
      red: 42,
      green: 42,
      blue: 42,
      null: 42,
    }
  });
}
