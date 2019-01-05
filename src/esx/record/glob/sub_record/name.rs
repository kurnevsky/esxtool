use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct GlobName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(glob_name_read_write, GlobName {
    name: String::from("42")
  });
}
