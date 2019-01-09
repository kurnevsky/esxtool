use crate::binary::*;

esx_enum! {
  enum FileType: u32 {
    Esp = 0,
    Esm = 1,
    Ess = 32
  }
}
