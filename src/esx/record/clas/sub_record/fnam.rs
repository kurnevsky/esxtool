use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_string! {
  struct ClasFnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(clas_fnam_read_write, ClasFnam {
    name: String::from("42")
  });
}
