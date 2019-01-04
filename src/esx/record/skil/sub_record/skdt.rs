use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;
use crate::esx::specialization::Specialization;

esx_sub_record_simple! {
  struct SkilSkdt {
    attribute: u32,
    specialization: Specialization,
    use_value: [f32; 4]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(skil_skdt_read_write, SkilSkdt {
    attribute: 42,
    specialization: Specialization::Combat,
    use_value: [42f32, 42f32, 42f32, 42f32]
  });
}
