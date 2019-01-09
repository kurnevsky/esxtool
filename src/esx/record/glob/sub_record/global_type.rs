use crate::binary::*;

esx_enum! {
  enum GlobalType: u8 {
    Short = b's',
    Long = b'l',
    Float = b'f'
  }
}
