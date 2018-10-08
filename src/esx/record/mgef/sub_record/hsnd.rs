use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct MgefHsnd(hit_sound)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_hsnd_read_write, MgefHsnd {
    hit_sound: String::from("42")
  });
}
