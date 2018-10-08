use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result, Write};

pub trait PrimitiveWrite : Sized {
  fn primitive_write<W: Write, B: ByteOrder>(self, output: &mut W) -> Result<()>;
}

pub trait PrimitiveRead : Sized {
  fn primitive_read<R: Read, B: ByteOrder>(input: &mut R) -> Result<Self>;
}

macro_rules! primitive_byte_write_impl {
  ( $type:ty, $write:ident ) => {
    impl PrimitiveWrite for $type {
      fn primitive_write<W: Write, B: ByteOrder>(self, output: &mut W) -> Result<()> {
          output.$write(self)
      }
    }
  }
}

macro_rules! primitive_byte_read_impl {
  ( $type:ty, $read:ident ) => {
    impl PrimitiveRead for $type {
      fn primitive_read<R: Read, B: ByteOrder>(input: &mut R) -> Result<Self> {
          input.$read()
      }
    }
  }
}

primitive_byte_write_impl!(u8, write_u8);
primitive_byte_write_impl!(i8, write_i8);

primitive_byte_read_impl!(u8, read_u8);
primitive_byte_read_impl!(i8, read_i8);

macro_rules! primitive_write_impl {
  ( $type:ty, $write:ident ) => {
    impl PrimitiveWrite for $type {
      fn primitive_write<W: Write, B: ByteOrder>(self, output: &mut W) -> Result<()> {
          output.$write::<B>(self)
      }
    }
  }
}

macro_rules! primitive_read_impl {
  ( $type:ty, $read:ident ) => {
    impl PrimitiveRead for $type {
      fn primitive_read<R: Read, B: ByteOrder>(input: &mut R) -> Result<Self> {
          input.$read::<B>()
      }
    }
  }
}

primitive_write_impl!(u16, write_u16);
primitive_write_impl!(u32, write_u32);
primitive_write_impl!(u64, write_u64);
primitive_write_impl!(i16, write_i16);
primitive_write_impl!(i32, write_i32);
primitive_write_impl!(i64, write_i64);
primitive_write_impl!(f32, write_f32);
primitive_write_impl!(f64, write_f64);

primitive_read_impl!(u16, read_u16);
primitive_read_impl!(u32, read_u32);
primitive_read_impl!(u64, read_u64);
primitive_read_impl!(i16, read_i16);
primitive_read_impl!(i32, read_i32);
primitive_read_impl!(i64, read_i64);
primitive_read_impl!(f32, read_f32);
primitive_read_impl!(f64, read_f64);
