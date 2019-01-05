use crate::binary::*;

esx_enum! {
  enum GlobalType: u8 {
    Short = b's',
    Long = b'l',
    Float = b'f'
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(global_type_short_read_write, GlobalType::Short);
  read_write_test!(global_type_long_read_write, GlobalType::Long);
  read_write_test!(global_type_float_read_write, GlobalType::Float);
}
