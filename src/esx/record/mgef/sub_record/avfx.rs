use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct MgefAvfx(area_visual)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_avfx_read_write, MgefAvfx {
    area_visual: String::from("42")
  });
}
