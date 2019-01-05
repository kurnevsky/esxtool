use crate::binary::*;

esx_sub_record_string! {
  struct ClasDesc(description)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(clas_desc_read_write, ClasDesc {
    description: String::from("42")
  });
}
