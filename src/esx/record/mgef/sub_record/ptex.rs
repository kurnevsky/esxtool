use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct MgefPtex(particle_texture)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_ptex_read_write, MgefPtex {
    particle_texture: String::from("42")
  });
}
