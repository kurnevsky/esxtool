use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct LtexName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(ltex_name_read_write, LtexName {
    name: String::from("42")
  });
}
