use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;

esx_enum! {
  enum FileType: u32 {
    Esp = 0,
    Esm = 1,
    Ess = 32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(file_type_esp_read_write, FileType::Esp);
  read_write_test!(file_type_esm_read_write, FileType::Esm);
  read_write_test!(file_type_ess_read_write, FileType::Ess);
}
