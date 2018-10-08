use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct MgefAsnd(area_sound)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_asnd_read_write, MgefAsnd {
    area_sound: String::from("42")
  });
}
