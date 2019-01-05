use super::global_type::GlobalType;
use crate::binary::*;

esx_sub_record_simple! {
  struct GlobFnam {
    global_type: GlobalType
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(glob_fnam_read_write, GlobFnam {
    global_type: GlobalType::Short
  });
}
