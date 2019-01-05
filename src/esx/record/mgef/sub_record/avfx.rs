use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct MgefAvfx(area_visual)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_avfx_read_write, MgefAvfx {
    area_visual: String::from("42")
  });
}
