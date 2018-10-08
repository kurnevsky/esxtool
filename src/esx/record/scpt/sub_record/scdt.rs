use std::io::{Read, Result, Seek, Write};

use binary::*;

esx_sub_record_vec! {
  struct ScptScdt(data)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(soun_name_read_write, ScptScdt {
    data: vec![42; 123],
  });
}
