use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct MgefHvfx(hit_visual)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_hvfx_read_write, MgefHvfx {
    hit_visual: String::from("42")
  });
}
