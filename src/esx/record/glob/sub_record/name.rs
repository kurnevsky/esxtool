use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct GlobName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(glob_name_read_write, GlobName {
    name: String::from("42")
  });
}
