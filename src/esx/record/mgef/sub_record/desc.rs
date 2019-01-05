use crate::binary::*;

esx_sub_record_string! {
  struct MgefDesc(description)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_desc_read_write, MgefDesc {
    description: String::from("42")
  });
}
