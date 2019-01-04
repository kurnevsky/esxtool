use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct MgefCsnd(cast_sound)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_csnd_read_write, MgefCsnd {
    cast_sound: String::from("42")
  });
}
