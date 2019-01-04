use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct MgefBsnd(bolt_sound)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_bsnd_read_write, MgefBsnd {
    bolt_sound: String::from("42")
  });
}
