use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct MgefItex(effect_icon)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_itex_read_write, MgefItex {
    effect_icon: String::from("42")
  });
}
