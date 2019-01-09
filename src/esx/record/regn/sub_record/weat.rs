use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use encoding::Encoding;

use crate::binary::*;
use crate::samples::Samples;

#[derive(Debug, Clone, PartialEq)]
pub struct RegnWeat {
  pub clear: u8,
  pub cloudy: u8,
  pub foggy: u8,
  pub overcast: u8,
  pub rain: u8,
  pub thunder: u8,
  pub ash: u8,
  pub blight: u8,
  pub snow: u8,
  pub blizard: u8,
}

impl Binary for RegnWeat {
  fn read<R: Read + Seek, E: Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
    let size = u32::read(input, encoding)?;
    if size != 8 && size != 10 {
      return Err(Error::new(ErrorKind::InvalidData, "Wrong size of RegnWeat"));
    }
    let clear = u8::read(input, encoding)?;
    let cloudy = u8::read(input, encoding)?;
    let foggy = u8::read(input, encoding)?;
    let overcast = u8::read(input, encoding)?;
    let rain = u8::read(input, encoding)?;
    let thunder = u8::read(input, encoding)?;
    let ash = u8::read(input, encoding)?;
    let blight = u8::read(input, encoding)?;
    let (snow, blizard) = if size == 8 {
      (0, 0)
    } else {
      (u8::read(input, encoding)?, u8::read(input, encoding)?)
    };
    Ok(RegnWeat {
      clear,
      cloudy,
      foggy,
      overcast,
      rain,
      thunder,
      ash,
      blight,
      snow,
      blizard,
    })
  }

  fn write<W: Write + Seek, E: Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
    let size = if self.snow == 0 && self.blizard == 0 {
      8u32
    } else {
      10u32
    };
    size.write(output, encoding)?;
    self.clear.write(output, encoding)?;
    self.cloudy.write(output, encoding)?;
    self.foggy.write(output, encoding)?;
    self.overcast.write(output, encoding)?;
    self.rain.write(output, encoding)?;
    self.thunder.write(output, encoding)?;
    self.ash.write(output, encoding)?;
    self.blight.write(output, encoding)?;
    if size == 10 {
      self.snow.write(output, encoding)?;
      self.blizard.write(output, encoding)?;
    }
    Ok(size + 4)
  }
}

#[cfg(test)]
impl Samples for RegnWeat {
  fn single() -> Self {
    RegnWeat {
      clear: 42,
      cloudy: 42,
      foggy: 42,
      overcast: 42,
      rain: 42,
      thunder: 42,
      ash: 42,
      blight: 42,
      snow: 42,
      blizard: 42,
    }
  }
  fn samples() -> Vec<Self> {
    vec![
      RegnWeat {
        clear: 42,
        cloudy: 42,
        foggy: 42,
        overcast: 42,
        rain: 42,
        thunder: 42,
        ash: 42,
        blight: 42,
        snow: 42,
        blizard: 42,
      },
      RegnWeat {
        clear: 42,
        cloudy: 42,
        foggy: 42,
        overcast: 42,
        rain: 42,
        thunder: 42,
        ash: 42,
        blight: 42,
        snow: 0,
        blizard: 0,
      }
    ]
  }
}

read_write_test!(RegnWeat);
