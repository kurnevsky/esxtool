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
pub mod misc;
pub mod weap;
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
use self::misc::MiscRecord;
use self::weap::WeapRecord;
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

    #[cfg(test)]
    impl crate::samples::Samples for Record {
      fn samples() -> Vec<Self> {
        vec![
          $(Record::$variant(<$value>::single()),)*
        ]
      }
      fn single() -> Self {
        first!($(Record::$variant(<$value>::single()),)*)
      }
    }

    read_write_test!(Record);
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
  Door(DoorRecord) => b"DOOR",
  Misc(MiscRecord) => b"MISC",
  Weap(WeapRecord) => b"WEAP"
}
