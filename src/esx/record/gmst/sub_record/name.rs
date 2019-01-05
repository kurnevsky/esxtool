use crate::binary::*;

esx_sub_record_string! {
  struct GmstName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(gmst_name_read_write, GmstName {
    name: String::from("42")
  });
}
