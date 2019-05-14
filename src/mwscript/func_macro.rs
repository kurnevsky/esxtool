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
  ( v ) => (ResType::Void);
}

macro_rules! arg_types {
  ( @build $($acc:expr),* ; ) => ([$($acc),*]);
  ( @build $($acc:expr),* ; | $($tail:tt)+ ) => (arg_types!(@build $($acc),*; $($tail)*));
  ( @build $($acc:expr),* ; $head:tt $($tail:tt)* ) => (arg_types!(@build $($acc,)* arg_type!($head); $($tail)*));
  ( $($arg:tt)* ) => (arg_types!(@build ; $($arg)*));
}

macro_rules! mwfuncs {
  ( $($name:ident ( $( $arg:tt )* ) : $res:tt ),* ) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Func {
      $($name),*
    }

    impl Func {
      pub fn by_name(s: &str) -> Option<Self> {
        FUNCS.get::<str>(&s.to_lowercase()).cloned()
      }

      pub fn name(self) -> &'static str {
        match self {
          $(
            Func::$name => stringify!($name)
          ),*
        }
      }

      pub fn required(self) -> u32 {
        match self {
          $(
            Func::$name => required_args!($($arg)*)
          ),*
        }
      }

      pub fn args(self) -> &'static [ArgType] {
        match self {
          $(
            Func::$name => &arg_types!($($arg)*)
          ),*
        }
      }

      pub fn result(self) -> ResType {
        match self {
          $(
            Func::$name => res_type!($res)
          ),*
        }
      }
    }
  }
}
