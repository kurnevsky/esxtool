pub mod record_flags;
pub mod tes3;
pub mod gmst;
pub mod glob;
pub mod clas;
pub mod fact;
pub mod race;
pub mod soun;
pub mod skil;
pub mod mgef;
pub mod scpt;
pub mod regn;
pub mod bsgn;
pub mod ltex;
pub mod stat;
pub mod door;
pub mod unknown;

use std::io::{Read, Result, Seek, SeekFrom, Write};
use encoding::Encoding;

use self::tes3::Tes3Record;
use self::gmst::GmstRecord;
use self::glob::GlobRecord;
use self::clas::ClasRecord;
use self::fact::FactRecord;
use self::race::RaceRecord;
use self::soun::SounRecord;
use self::skil::SkilRecord;
use self::mgef::MgefRecord;
use self::scpt::ScptRecord;
use self::regn::RegnRecord;
use self::bsgn::BsgnRecord;
use self::ltex::LtexRecord;
use self::stat::StatRecord;
use self::door::DoorRecord;
use self::unknown::UnknownRecord;
use crate::binary::*;
use crate::esx::util::name_to_string;

macro_rules! record {
  ( $( $variant:ident ( $value:ty ) => $name:expr ),* ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Record {
      $(
        $variant($value),
      )*
      Unknown(UnknownRecord),
    }

    impl Record {
      pub fn name(&self) -> [u8; 4] {
        match self {
          $(
            Record::$variant(_) => *$name,
          )*
            Record::Unknown(unknown) => unknown.name,
        }
      }
    }

    impl Binary for Record {
      fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
        let name = read_name(input)?;
        trace!("Read record {}", name_to_string(name));
        match &name {
          $(
            $name => <$value>::read(input, encoding).map(Record::$variant),
          )*
            _ => {
              input.seek(SeekFrom::Current(-4))?;
              UnknownRecord::read(input, encoding).map(Record::Unknown)
            },
        }
      }

      fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
        let len = match self {
          $(
            Record::$variant(value) => {
              output.write_all($name)?;
              value.write(output, encoding)?
            },
          )*
            Record::Unknown(unknown) => {
              unknown.write(output, encoding)? - 4
            },
        };
        Ok(len + 4)
      }
    }
  }
}

record! {
  Tes3(Tes3Record) => b"TES3",
  Gmst(GmstRecord) => b"GMST",
  Glob(GlobRecord) => b"GLOB",
  Clas(ClasRecord) => b"CLAS",
  Fact(FactRecord) => b"FACT",
  Race(RaceRecord) => b"RACE",
  Soun(SounRecord) => b"SOUN",
  Skil(SkilRecord) => b"SKIL",
  Mgef(MgefRecord) => b"MGEF",
  Scpt(ScptRecord) => b"SCPT",
  Regn(RegnRecord) => b"REGN",
  Bsgn(BsgnRecord) => b"BSGN",
  Ltex(LtexRecord) => b"LTEX",
  Stat(StatRecord) => b"STAT",
  Door(DoorRecord) => b"DOOR"
}

#[cfg(test)]
mod tests {
  use super::*;

  use super::record_flags::RecordFlags;
  use super::tes3::sub_record::*;
  use super::unknown::*;

  read_write_test!(record_tes3_read_write, Record::Tes3(Tes3Record {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      Tes3SubRecord::Hedr(Tes3Hedr {
        version: 42f32,
        file_type: FileType::Esp,
        company_name: String::from("42"),
        file_description: String::from("43"),
        num_records: 43
      }),
      Tes3SubRecord::Mast(Tes3Mast {
        master_file_name: String::from("42")
      }),
      Tes3SubRecord::Data(Tes3Data {
        master_size: 42
      }),
    ],
  }));

  read_write_test!(record_unknown_read_write, Record::Unknown(UnknownRecord {
    name: *b"ABCD",
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      UnknownSubRecord {
        name: *b"ABCD",
        data: vec![42; 123],
      }
    ],
  }));
}
