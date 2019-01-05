mod file_type;
mod hedr;
mod mast;
mod data;

pub use self::file_type::*;
pub use self::hedr::*;
pub use self::mast::*;
pub use self::data::*;

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum Tes3SubRecord {
    Hedr(Tes3Hedr) => b"HEDR",
    Mast(Tes3Mast) => b"MAST",
    Data(Tes3Data) => b"DATA"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(tes3_sub_record_data_read_write, Tes3SubRecord::Data(Tes3Data {
    master_size: 42
  }));

  read_write_test!(tes3_sub_record_mast_read_write, Tes3SubRecord::Mast(Tes3Mast {
    master_file_name: String::from("42")
  }));

  read_write_test!(tes3_sub_record_hedr_read_write, Tes3SubRecord::Hedr(Tes3Hedr {
    version: 42f32,
    file_type: FileType::Esp,
    company_name: String::from("42"),
    file_description: String::from("43"),
    num_records: 43
  }));
}
