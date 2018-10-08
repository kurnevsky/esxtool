use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_string! {
  struct RaceFnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(race_fnam_read_write, RaceFnam {
    name: String::from("42")
  });
}
