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
      fn read<R: Read + Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
        let v = <$enum_type>::read(input, encoding)?;
        match v {
          $(
            v if v == $number => Ok($name::$variant),
          )*
            v => Err(Error::new(ErrorKind::InvalidData, format!("Wrong value of {}: {}", stringify!($name), v)))
        }
      }

      fn write<W: Write + Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
        use std::mem::size_of;
        let v: $enum_type = (*self).into();
        v.write(output, encoding)?;
        Ok(size_of::<$enum_type>() as u32)
      }
    }
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
      fn read<R: Read + Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
        let size = u32::read(input, encoding)?;
        let unknown = u32::read(input, encoding)?;
        let flags = RecordFlags::read(input, encoding)?;
        let pos = input.seek(SeekFrom::Current(0))?;
        let mut cur_pos = pos;
        let mut sub_records = Vec::new();
        while size > (cur_pos - pos) as u32 {
          sub_records.push(<$sub_record>::read(input, encoding)?);
          cur_pos = input.seek(SeekFrom::Current(0))?;
        }
        if size != (cur_pos - pos) as u32 {
          return Err(Error::new(ErrorKind::InvalidData, format!("Wrong length of {} record: {}", stringify!($record), size)));
        }
        Ok($record {
          unknown,
          flags,
          sub_records,
        })
      }

      fn write<W: Write + Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
        let pos = output.seek(SeekFrom::Current(4))?;
        self.unknown.write(output, encoding)?;
        self.flags.write(output, encoding)?;
        let mut len = 0;
        for sub_record in &self.sub_records {
          len += sub_record.write(output, encoding)?;
        }
        output.seek(SeekFrom::Start(pos - 4))?;
        len.write(output, encoding)?;
        output.seek(SeekFrom::End(0))?;
        Ok(len + 12)
      }
    }
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
      fn read<R: Read + Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
        let name = read_name(input)?;
        match &name {
          $(
            $name => <$value>::read(input, encoding).map($sub_record::$variant),
          )*
            other => Err(Error::new(ErrorKind::InvalidData, format!("Invalid sub record {} for {}", name_to_string(*other), stringify!($sub_record)))),
        }
      }

      fn write<W: Write + Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
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
  }
}

macro_rules! total_size {
  () => (0);
  ( $head:ty ) => (size_of::<$head>() as u32);
  ( $head:ty , $($tail:ty),+ ) => (total_size!($head) + total_size!($($tail),*));
}

macro_rules! esx_data {
  ( struct $name:ident { $($field:ident : $field_type:ty ),* } ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $name {
      $(pub $field: $field_type),*
    }

    impl Binary for $name {
      fn read<R: Read + Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
        $(
          let $field = <$field_type>::read(input, encoding)?;
        )*
        Ok($name {
          $($field),*
        })
      }

      fn write<W: Write + Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
        use std::mem::size_of;
        $(
          self.$field.write(output, encoding)?;
        )*
        Ok(total_size!($($field_type),*))
      }
    }
  }
}

macro_rules! esx_sub_record_simple {
  ( struct $name:ident { $($field:ident : $field_type:ty ),* } ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $name {
      $(pub $field: $field_type),*
    }

    impl Binary for $name {
      fn read<R: Read + Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
        use std::mem::size_of;
        let size = u32::read(input, encoding)?;
        if size != total_size!($($field_type),*) {
          return Err(Error::new(ErrorKind::InvalidData, format!("Wrong size of {}: {}", stringify!($name), size)));
        }
        $(
          let $field = <$field_type>::read(input, encoding)?;
        )*
        Ok($name {
          $($field),*
        })
      }

      fn write<W: Write + Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
        use std::mem::size_of;
        total_size!($($field_type),*).write(output, encoding)?;
        $(
          self.$field.write(output, encoding)?;
        )*
        Ok(total_size!($($field_type),*) + 4)
      }
    }
  }
}

macro_rules! esx_sub_record_vec {
  ( struct $name:ident ( $field:ident ) ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $name {
      pub $field: Vec<u8>
    }

    impl Binary for $name {
      fn read<R: Read + Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
        let size = u32::read(input, encoding)?;
        let mut $field = vec![0; size as usize];
        input.read_exact(&mut $field)?;
        Ok($name {
          $field
        })
      }

      fn write<W: Write + Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
        (self.$field.len() as u32).write(output, encoding)?;
        output.write_all(&self.$field)?;
        Ok(self.$field.len() as u32 + 4)
      }
    }
  }
}

macro_rules! esx_sub_record_string {
  ( struct $name:ident ( $field:ident ) ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $name {
      pub $field: String
    }

    impl Binary for $name {
      fn read<R: Read + Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
        let size = u32::read(input, encoding)?;
        let $field = read_string(input, encoding, size)?;
        Ok($name {
          $field
        })
      }

      fn write<W: Write + Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
        let buf = encoding.encode(&self.$field, encoding::EncoderTrap::Strict).map_err(|e|
            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        )?;
        let len = buf.len() as u32;
        len.write(output, encoding)?;
        output.write_all(&buf)?;
        Ok(len + 4)
      }
    }
  }
}

macro_rules! esx_sub_record_fixed_string {
  ( struct $name:ident ( $field:ident : $size:expr ) ) => {
    #[derive(Debug, Clone, PartialEq)]
    pub struct $name {
      pub $field: String
    }

    impl Binary for $name {
      fn read<R: Read + Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
        let size = u32::read(input, encoding)?;
        if size != $size {
          return Err(Error::new(ErrorKind::InvalidData, format!("Wrong size of {}: {}", stringify!($name), size)));
        }
        let $field = read_string(input, encoding, size)?;
        Ok($name {
          $field
        })
      }

      fn write<W: Write + Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
        u32::write(&$size, output, encoding)?;
        write_string_exact(output, encoding, &self.$field, $size)?;
        Ok($size + 4)
      }
    }
  }
}

macro_rules! bitflags_binary {
  ( $name:ident ) => {
    impl Binary for $name {
      fn read<R: Read + Seek, E: encoding::Encoding>(input: &mut R, encoding: &E) -> Result<Self> {
        let flags = Binary::read(input, encoding)?;
        $name::from_bits(flags).ok_or_else(||
          Error::new(ErrorKind::InvalidData, format!("Wrong flags of {} record: {}", stringify!($name), flags))
        )
      }

      fn write<W: Write + Seek, E: encoding::Encoding>(&self, output: &mut W, encoding: &E) -> Result<u32> {
        self.bits().write(output, encoding)
      }
    }
  }
}
