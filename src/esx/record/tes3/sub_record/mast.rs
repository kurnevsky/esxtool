use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct Tes3Mast(master_file_name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(tes3_mast_read_write, Tes3Mast {
    master_file_name: String::from("42")
  });
}
