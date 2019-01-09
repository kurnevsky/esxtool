pub trait Samples: Sized {
  fn samples() -> Vec<Self> {
    vec![Self::single()]
  }
  fn single() -> Self;
}

macro_rules! unsigned_samples {
  ( $type:ty ) => {
    impl Samples for $type {
      fn samples() -> Vec<Self> {
        vec![0, 1, 2, 42, <$type>::max_value()]
      }
      fn single() -> Self {
        42
      }
    }
  }
}

unsigned_samples!(u8);
unsigned_samples!(u16);
unsigned_samples!(u32);
unsigned_samples!(u64);

macro_rules! signed_samples {
  ( $type:ty ) => {
    impl Samples for $type {
      fn samples() -> Vec<Self> {
        vec![0, 1, 2, 42, <$type>::max_value(), -1, <$type>::min_value()]
      }
      fn single() -> Self {
        42
      }
    }
  }
}

signed_samples!(i8);
signed_samples!(i16);
signed_samples!(i32);
signed_samples!(i64);

macro_rules! float_samples {
  ( $type:ty ) => {
    impl Samples for $type {
      fn samples() -> Vec<Self> {
        vec![0.0, 1.0, 2.0, 42.0, -1.0]
      }
      fn single() -> Self {
        42.0
      }
    }
  }
}

float_samples!(f32);
float_samples!(f64);

macro_rules! array_samples {
  ( [$($acc:expr,)*] $n:expr ) => {
    impl<T: Samples> Samples for [T; $n] {
      fn single() -> Self {
        [
          $(
            {
              let _ = $acc;
              Samples::single()
            }
          ),*
        ]
      }
    }
  }
}

macro_rules! arrays_samples {
  ( [$($acc:expr,)*] ) => { };
  ( [$($acc:expr,)*] $h:expr, $($n:expr,)* ) => {
    array_samples!([$($acc,)*] $h);
    arrays_samples!([$($acc,)* $h,] $($n,)*);
  };
  ( $($n:expr)* ) => {
    arrays_samples!([] $($n,)*);
  }
}

arrays_samples!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15);
