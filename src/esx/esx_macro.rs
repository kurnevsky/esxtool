macro_rules! first {
  ($first:expr, $($rest:expr,)*) => { $first };
}

macro_rules! esx_enum {
  ( enum $name:ident : $enum_type:ident { $( $variant:ident = $number:expr ),* } ) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr($enum_type)]
    pub enum $name {
      $(
        $variant,
      )*
    }

    impl Into<$enum_type> for $name {
      fn into(self) -> $enum_type {
        match self {
          $(
            $name::$variant => $number,
          )*
        }
      }
    }

    impl Binary for $name {
      fn read<R: std::io::Read + std::io::Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> std::io::Result<Self> {
        let v = <$enum_type>::read(input, encoding)?;
        match v {
          $(
            v if v == $number => Ok($name::$variant),
          )*
            v => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Wrong value of {}: {}", stringify!($name), v)))
        }
      }

      fn write<W: std::io::Write + std::io::Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> std::io::Result<u32> {
        use std::mem::size_of;
        let v: $enum_type = (*self).into();
        v.write(output, encoding)?;
        Ok(size_of::<$enum_type>() as u32)
      }
    }

    #[cfg(test)]
    impl crate::samples::Samples for $name {
      fn samples() -> Vec<Self> {
        vec![
          $($name::$variant),*
        ]
      }
      fn single() -> Self {
        first!($($name::$variant,)*)
      }
    }

    read_write_test!($name);
  }
}

macro_rules! esx_record {
  ( $record:ident, $sub_record:ident ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $record {
      pub unknown: u32,
      pub flags: RecordFlags,
      pub sub_records: Vec<$sub_record>,
    }

    impl Binary for $record {
      fn read<R: std::io::Read + std::io::Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> std::io::Result<Self> {
        let size = u32::read(input, encoding)?;
        let unknown = u32::read(input, encoding)?;
        let flags = RecordFlags::read(input, encoding)?;
        let pos = input.seek(std::io::SeekFrom::Current(0))?;
        let mut cur_pos = pos;
        let mut sub_records = Vec::new();
        while size > (cur_pos - pos) as u32 {
          sub_records.push(<$sub_record>::read(input, encoding)?);
          cur_pos = input.seek(std::io::SeekFrom::Current(0))?;
        }
        if size != (cur_pos - pos) as u32 {
          return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Wrong length of {} record: {}", stringify!($record), size)));
        }
        Ok($record {
          unknown,
          flags,
          sub_records,
        })
      }

      fn write<W: std::io::Write + std::io::Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> std::io::Result<u32> {
        let pos = output.seek(std::io::SeekFrom::Current(4))?;
        self.unknown.write(output, encoding)?;
        self.flags.write(output, encoding)?;
        let mut len = 0;
        for sub_record in &self.sub_records {
          len += sub_record.write(output, encoding)?;
        }
        output.seek(std::io::SeekFrom::Start(pos - 4))?;
        len.write(output, encoding)?;
        output.seek(std::io::SeekFrom::End(0))?;
        Ok(len + 12)
      }
    }

    #[cfg(test)]
    impl crate::samples::Samples for $record {
      fn single() -> Self {
        $record {
          unknown: 0,
          flags: RecordFlags::Persistent,
          sub_records: crate::samples::Samples::samples(),
        }
      }
    }

    read_write_test!($record);
  }
}

macro_rules! esx_sub_record {
  ( enum $sub_record:ident { $( $variant:ident ( $value:ty ) => $name:expr ),* } ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub enum $sub_record {
      $(
        $variant($value),
      )*
    }

    impl $sub_record {
      pub fn name(&self) -> [u8; 4] {
        match self {
          $(
            $sub_record::$variant(_) => *$name,
          )*
        }
      }
    }

    impl Binary for $sub_record {
      fn read<R: std::io::Read + std::io::Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> std::io::Result<Self> {
        let name = read_name(input)?;
        trace!("Read subrecord {}", name_to_string(name));
        match &name {
          $(
            $name => <$value>::read(input, encoding).map($sub_record::$variant),
          )*
            other => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid sub record {} for {}", name_to_string(*other), stringify!($sub_record)))),
        }
      }

      fn write<W: std::io::Write + std::io::Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> std::io::Result<u32> {
        let len = match self {
          $(
            $sub_record::$variant(value) => {
              output.write_all($name)?;
              value.write(output, encoding)?
            },
          )*
        };
        Ok(len + 4)
      }
    }

    #[cfg(test)]
    impl crate::samples::Samples for $sub_record {
      fn samples() -> Vec<Self> {
        vec![
          $($sub_record::$variant(<$value>::single()),)*
        ]
      }
      fn single() -> Self {
        first!($($sub_record::$variant(<$value>::single()),)*)
      }
    }

    read_write_test!($sub_record);
  }
}

macro_rules! total_size {
  () => (0);
  ( $head:ty ) => (size_of::<$head>() as u32);
  ( $head:ty , $($tail:ty),+ ) => (total_size!($head) + total_size!($($tail),*));
}

macro_rules! esx_data {
  (@inner $name:ident $vec:ident [$($left:ident,)*] []) => { };
  (@inner $name:ident $vec:ident [$($left:ident,)*] [$current:ident, $($right:ident,)*]) => {
    for value in crate::samples::Samples::samples() {
      $vec.push($name {
        $($left: crate::samples::Samples::single(),)*
          $current: value,
          $($right: crate::samples::Samples::single(),)*
      });
    }
    esx_data!(@inner $name $vec [$($left,)* $current,] [$($right,)*])
  };

  ( struct $name:ident { $($field:ident : $field_type:ty ),* } ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $name {
      $(pub $field: $field_type),*
    }

    impl Binary for $name {
      fn read<R: std::io::Read + std::io::Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> std::io::Result<Self> {
        $(
          let $field = <$field_type>::read(input, encoding)?;
        )*
        Ok($name {
          $($field),*
        })
      }

      fn write<W: std::io::Write + std::io::Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> std::io::Result<u32> {
        use std::mem::size_of;
        $(
          self.$field.write(output, encoding)?;
        )*
        Ok(total_size!($($field_type),*))
      }
    }

    #[cfg(test)]
    impl crate::samples::Samples for $name {
      fn samples() -> Vec<Self> {
        let mut vec = Vec::new();
        esx_data!(@inner $name vec [] [$($field,)*]);
        vec
      }
      fn single() -> Self {
        $name {
          $($field: crate::samples::Samples::single()),*
        }
      }
    }

    read_write_test!($name);
  }
}

macro_rules! esx_sub_record_simple {
  (@inner $name:ident $vec:ident [$($left:ident,)*] []) => { };
  (@inner $name:ident $vec:ident [$($left:ident,)*] [$current:ident, $($right:ident,)*]) => {
    for value in crate::samples::Samples::samples() {
      $vec.push($name {
        $($left: crate::samples::Samples::single(),)*
          $current: value,
          $($right: crate::samples::Samples::single(),)*
      });
    }
    esx_sub_record_simple!(@inner $name $vec [$($left,)* $current,] [$($right,)*])
  };

  ( struct $name:ident { $($field:ident : $field_type:ty ),* } ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $name {
      $(pub $field: $field_type),*
    }

    impl Binary for $name {
      fn read<R: std::io::Read + std::io::Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> std::io::Result<Self> {
        use std::mem::size_of;
        let size = u32::read(input, encoding)?;
        if size != total_size!($($field_type),*) {
          return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Wrong size of {}: {}", stringify!($name), size)));
        }
        $(
          let $field = <$field_type>::read(input, encoding)?;
        )*
        Ok($name {
          $($field),*
        })
      }

      fn write<W: std::io::Write + std::io::Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> std::io::Result<u32> {
        use std::mem::size_of;
        total_size!($($field_type),*).write(output, encoding)?;
        $(
          self.$field.write(output, encoding)?;
        )*
        Ok(total_size!($($field_type),*) + 4)
      }
    }

    #[cfg(test)]
    impl crate::samples::Samples for $name {
      fn samples() -> Vec<Self> {
        let mut vec = Vec::new();
        esx_sub_record_simple!(@inner $name vec [] [$($field,)*]);
        vec
      }
      fn single() -> Self {
        $name {
          $($field: crate::samples::Samples::single()),*
        }
      }
    }

    read_write_test!($name);
  }
}

macro_rules! esx_sub_record_vec {
  ( struct $name:ident ( $field:ident ) ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $name {
      pub $field: Vec<u8>
    }

    impl Binary for $name {
      fn read<R: std::io::Read + std::io::Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> std::io::Result<Self> {
        let size = u32::read(input, encoding)?;
        let mut $field = vec![0; size as usize];
        input.read_exact(&mut $field)?;
        Ok($name {
          $field
        })
      }

      fn write<W: std::io::Write + std::io::Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> std::io::Result<u32> {
        (self.$field.len() as u32).write(output, encoding)?;
        output.write_all(&self.$field)?;
        Ok(self.$field.len() as u32 + 4)
      }
    }

    #[cfg(test)]
    impl crate::samples::Samples for $name {
      fn single() -> Self {
        $name {
          $field: vec![42; 123]
        }
      }
    }

    read_write_test!($name);
  }
}

macro_rules! esx_sub_record_string {
  ( struct $name:ident ( $field:ident ) ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $name {
      pub $field: String
    }

    impl Binary for $name {
      fn read<R: std::io::Read + std::io::Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> std::io::Result<Self> {
        let size = u32::read(input, encoding)? as usize;
        let mut buf = vec![0; size];
        input.read_exact(&mut buf)?;
        let size_not_null = buf.iter().rposition(|&c| c != 0).map_or(0, |idx| idx + 1);
        if size_not_null != size {
          warn!("Got null terminated {}", stringify!($name));
        }
        let $field = encoding.decode(&buf[0..size_not_null], encoding::DecoderTrap::Strict).map_err(|e|
          std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        )?;
        Ok($name {
          $field
        })
      }

      fn write<W: std::io::Write + std::io::Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> std::io::Result<u32> {
        let buf = encoding.encode(&self.$field, encoding::EncoderTrap::Strict).map_err(|e|
            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        )?;
        let len = buf.len() as u32;
        len.write(output, encoding)?;
        output.write_all(&buf)?;
        Ok(len + 4)
      }
    }

    #[cfg(test)]
    impl crate::samples::Samples for $name {
      fn single() -> Self {
        $name {
          $field: String::from("42")
        }
      }
    }

    read_write_test!($name);
  }
}

macro_rules! esx_sub_record_null_terminated_string {
  ( struct $name:ident ( $field:ident ) ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $name {
      pub $field: String
    }

    impl Binary for $name {
      fn read<R: std::io::Read + std::io::Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> std::io::Result<Self> {
        let size = u32::read(input, encoding)? as usize;
        let mut buf = vec![0; size];
        input.read_exact(&mut buf)?;
        let size_not_null = buf.iter().rposition(|&c| c != 0).map_or(0, |idx| idx + 1);
        if size_not_null == size {
          warn!("Got not null terminated {}", stringify!($name));
        }
        let $field = encoding.decode(&buf[0..size_not_null], encoding::DecoderTrap::Strict).map_err(|e|
          std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        )?;
        Ok($name {
          $field
        })
      }

      fn write<W: std::io::Write + std::io::Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> std::io::Result<u32> {
        use byteorder::WriteBytesExt;
        let buf = encoding.encode(&self.$field, encoding::EncoderTrap::Strict).map_err(|e|
            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        )?;
        let len = buf.len() as u32 + 1;
        len.write(output, encoding)?;
        output.write_all(&buf)?;
        output.write_u8(0)?;
        Ok(len + 4)
      }
    }

    #[cfg(test)]
    impl crate::samples::Samples for $name {
      fn single() -> Self {
        $name {
          $field: String::from("42")
        }
      }
    }

    read_write_test!($name);
  }
}

macro_rules! esx_sub_record_fixed_string {
  ( struct $name:ident ( $field:ident : $size:expr ) ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $name {
      pub $field: String
    }

    impl Binary for $name {
      fn read<R: std::io::Read + std::io::Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> std::io::Result<Self> {
        let size = u32::read(input, encoding)?;
        if size != $size {
          return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Wrong size of {}: {}", stringify!($name), size)));
        }
        let $field = read_string(input, encoding, size)?;
        Ok($name {
          $field
        })
      }

      fn write<W: std::io::Write + std::io::Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> std::io::Result<u32> {
        u32::write(&$size, output, encoding)?;
        write_string_exact(output, encoding, &self.$field, $size)?;
        Ok($size + 4)
      }
    }

    #[cfg(test)]
    impl crate::samples::Samples for $name {
      fn single() -> Self {
        $name {
          $field: String::from("42")
        }
      }
    }

    read_write_test!($name);
  }
}

macro_rules! esx_bitflags {
  (
    struct $name:ident : $type:ty {
      $(
        const $flag:ident = $value:expr;
      )+
    }
  ) => {
    bitflags! {
      pub struct $name: $type {
        $(
          const $flag = $value;
        )*
      }
    }

    impl Binary for $name {
      fn read<R: std::io::Read + std::io::Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> std::io::Result<Self> {
        let flags = Binary::read(input, encoding)?;
        $name::from_bits(flags).ok_or_else(||
          std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Wrong flags of {} record: {}", stringify!($name), flags))
        )
      }

      fn write<W: std::io::Write + std::io::Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> std::io::Result<u32> {
        self.bits().write(output, encoding)
      }
    }

    #[cfg(test)]
    impl crate::samples::Samples for $name {
      fn samples() -> Vec<Self> {
        vec![
          $($name::$flag),*
        ]
      }
      fn single() -> Self {
        first!($($name::$flag,)*)
      }
    }

    read_write_test!($name);
  };
}
