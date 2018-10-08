macro_rules! required_args {
  () => (0);
  ( | $( $arg:tt )+ ) => (0);
  ( $head:tt $($tail:tt)* ) => (1 + required_args!($($tail)*));
}

macro_rules! arg_type {
  ( f ) => (ArgType::Float);
  ( l ) => (ArgType::Long);
  ( s ) => (ArgType::String);
}

macro_rules! res_type {
  ( f ) => (ResType::Float);
  ( l ) => (ResType::Long);
}

macro_rules! arg_types {
  ( @build $($acc:expr),* ; ) => ([$($acc),*]);
  ( @build $($acc:expr),* ; | $($tail:tt)+ ) => (arg_types!(@build $($acc),*; $($tail)*));
  ( @build $($acc:expr),* ; $head:tt $($tail:tt)* ) => (arg_types!(@build $($acc,)* arg_type!($head); $($tail)*));
  ( $($arg:tt)* ) => (arg_types!(@build ; $($arg)*));
}

macro_rules! by_name_impl {
  ( $map:ident ) => {
    pub fn by_name(s: &str) -> Option<Self> {
      $map.get::<str>(&s.to_lowercase()).cloned()
    }
  }
}

macro_rules! name_impl {
  ( $enum:ident , $( $name:ident )* ) => {
    pub fn name(self) -> &'static str {
      match self {
        $(
          $enum::$name => stringify!($name)
        ),*
      }
    }
  }
}

macro_rules! required_impl {
  ( $enum:ident , $( $name:ident ( $( $arg:tt )* ) )* ) => {
    pub fn required(self) -> u32 {
      match self {
        $(
          $enum::$name => required_args!($($arg)*)
        ),*
      }
    }
  }
}

macro_rules! args_impl {
  ( $enum:ident , $( $name:ident ( $( $arg:tt )* ) )* ) => {
    pub fn args(self) -> &'static [ArgType] {
      match self {
        $(
          $enum::$name => &arg_types!($($arg)*)
        ),*
      }
    }
  }
}

macro_rules! result_impl {
  ( $enum:ident , $( $name:ident : $res:tt )* ) => {
    pub fn result(self) -> ResType {
      match self {
        $(
          $enum::$name => res_type!($res)
        ),*
      }
    }
  }
}

macro_rules! mwprocs {
  ( $($name:ident ( $( $arg:tt )* ) ),* ) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Proc {
      $($name),*
    }

    impl Proc {
      by_name_impl!(PROCS);
      name_impl!(Proc, $($name)*);
      required_impl!(Proc, $($name($($arg)*))*);
      args_impl!(Proc, $($name($($arg)*))*);
    }
  }
}

macro_rules! mwfuncs {
  ( $($name:ident ( $( $arg:tt )* ) : $res:tt ),* ) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Func {
      $($name),*
    }

    impl Func {
      by_name_impl!(FUNCS);
      name_impl!(Func, $($name)*);
      required_impl!(Func, $($name($($arg)*))*);
      args_impl!(Func, $($name($($arg)*))*);
      result_impl!(Func, $($name: $res)*);
    }
  }
}
