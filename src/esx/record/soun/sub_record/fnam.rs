use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct SounFnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(soun_fnam_read_write, SounFnam {
    name: String::from("42")
  });
}
