use std::io::{Read, Result, Seek, Write};

use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct ClasName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(clas_name_read_write, ClasName {
    name: String::from("42")
  });
}
