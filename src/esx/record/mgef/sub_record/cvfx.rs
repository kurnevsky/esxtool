use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct MgefCvfx(casting_visual)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_cvfx_read_write, MgefCvfx {
    casting_visual: String::from("42")
  });
}