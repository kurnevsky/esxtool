use combine::*;
use combine::parser::char::*;
use combine::parser::combinator::factory;
use combine::parser::range::*;

use super::script::*;
use super::funcs::*;

fn keyword<I>(s: &'static str) -> impl Parser<Input = I, Output = &'static str>
where
  I: Stream<Item = char>,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{ // todo: skip spaces here?
  attempt(
    string_cmp(s, |l, r| l.eq_ignore_ascii_case(&r))
      .skip(not_followed_by(alpha_num().or(char('_'))))
  )
}

fn comment<I>() -> impl Parser<Input = I, Output = ()>
where
  I: Stream<Item = char>,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  (token(';'), skip_many(satisfy(|c| c != '\n'))).map(|_| ())
}

fn whitespace<I>() -> impl Parser<Input = I, Output = ()>
where
  I: Stream<Item = char>,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  let space = token(' ').or(tab());
  skip_many(space)
}

fn lineend<I>() -> impl Parser<Input = I, Output = ()>
where
  I: Stream<Item = char>,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  let lineend = (
    whitespace(),
    optional(comment()),
    crlf().or(newline()),
    whitespace(),
  );
  skip_many1(lineend)
}

fn number<'a, I>() -> impl Parser<Input = I, Output = Expr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  let guard = look_ahead(optional(char('.')).with(digit()));
  let parser = (
    take_while(|c: char| c.is_numeric()),
    optional(char('.')),
    take_while(|c: char| c.is_numeric()),
  ).map(|(int, dot, fract): (&str, Option<char>, &str)|
    if dot.is_some() {
        let mut value = 0f32;
        for d in int.chars().map(|c| c.to_digit(10).unwrap()) {
            value = value * 10f32 + d as f32;
        }
        for (i, d) in fract.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            value += (d as f32) * 10f32.powf(-(i as f32) - 1f32);
        }
        Expr::Float(value)
    } else {
        let mut value = 0i32;
        for d in int.chars().map(|c| c.to_digit(10).unwrap()) {
            value = value.saturating_mul(10).saturating_add(d as i32);
        }
        Expr::Long(value)
    }
  ).skip(whitespace());
  guard.with(parser)
}

fn arg<'a, I>(arg_type: ArgType) -> impl Parser<Input = I, Output = Arg> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  match arg_type {
    ArgType::String => {
      id().map(Arg::Id).left()
    },
    ArgType::Float | ArgType::Long => {
      let opaque_muldiv = parser(|i| muldiv().parse_stream(i));
      opaque_muldiv.map(Arg::Expr).right()
    },
  }
}

fn args<'a, I>(args: &'static [ArgType], required: u32) -> impl Parser<Input = I, Output = Vec<Arg>> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  if args.is_empty() {
    return value(Vec::new()).left();
  }
  let mut args_iter = args.into_iter();
  let comma = || optional(char(',').skip(whitespace()));
  let arg = factory(move || match args_iter.next() {
    Some(&arg_type) => arg(arg_type).skip(comma()).left(),
    None => unexpected_any("Wrong arg number").right(),
  });
  comma().with(count_min_max(
    required as usize,
    args.len(),
    arg,
  )).right()
}

fn fix<'a, I>() -> impl Parser<Input = I, Output = (Option<String>, String)> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  (
    id(),
    optional(
      attempt(string("->"))
        .skip(whitespace())
        .with(id())
    )
  ).map(|(fix, id)| match id {
    Some(id) => (Some(fix), id),
    None => (None, fix),
  })
}

fn func<'a, I>() -> impl Parser<Input = I, Output = Expr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  fix().then(|(fix, name)| match Func::by_name(&name) {
    Some(func) => {
      // TODO: error when func.result() is ResType::Void?
      (
        value(fix),
        args(func.args(), func.required())
      ).map(move |(fix, args)|
        Expr::Func(fix, func, args)
      ).left()
    },
    None => {
      (
        value(name),
        optional(
          char('.')
            .skip(whitespace())
            .with(id())
        )
      ).map(|(name, var_name)| match var_name {
        Some(var_name) => Expr::Var(Some(name), var_name),
        None => Expr::Var(None, name),
      }).right()
    },
  })
}

fn brackets<'a, I>() -> impl Parser<Input = I, Output = Expr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  let opaque_expr = || parser(|i| expr().parse_stream(i));
  let char_ws = |c| char(c).skip(whitespace());
  between(char_ws('('), char_ws(')'), opaque_expr())
    .or(between(char_ws('['), char_ws(']'), opaque_expr()))
    .map(|expr| Expr::Brackets(Box::new(expr)))
    .or(number())
    .or(func())
}

fn minusplus<'a, I>() -> impl Parser<Input = I, Output = Expr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  let opaque_minusplus = parser(|i| minusplus().parse_stream(i));
  let op = char('-')
    .map(|_| UnaryOp::Minus)
    .or(char('+').map(|_| UnaryOp::Plus))
    .skip(whitespace());
  (op, opaque_minusplus)
    .map(|(op, expr)| Expr::UnaryOp(op, Box::new(expr)))
    .or(brackets())
}

fn muldiv<'a, I>() -> impl Parser<Input = I, Output = Expr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  let op = char('*')
    .map(|_| BinaryOp::Mul)
    .or(char('/').map(|_| BinaryOp::Div))
    .skip(whitespace());
  (minusplus(), many((op, minusplus()))).map(|(h, t): (Expr, Vec<(BinaryOp, Expr)>)|
    t.into_iter().fold(h, |acc, (op, expr)|
      Expr::BinaryOp(op, Box::new(acc), Box::new(expr))
    )
  )
}

fn addsub<'a, I>() -> impl Parser<Input = I, Output = Expr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  let op = char('+')
    .map(|_| BinaryOp::Add)
    .or(char('-').map(|_| BinaryOp::Sub))
    .skip(whitespace());
  (muldiv(), many((op, muldiv()))).map(|(h, t): (Expr, Vec<(BinaryOp, Expr)>)|
    t.into_iter().fold(h, |acc, (op, expr)|
      Expr::BinaryOp(op, Box::new(acc), Box::new(expr))
    )
  )
}

fn comparison<'a, I>() -> impl Parser<Input = I, Output = Expr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  let op = choice((
    attempt(string("= =")).map(|_| BinaryOp::Eq),
    attempt(string("==")).map(|_| BinaryOp::Eq),
    char('=').map(|_| BinaryOp::Eq),
    attempt(string("! =")).map(|_| BinaryOp::NotEq),
    attempt(string("!=")).map(|_| BinaryOp::NotEq),
    attempt(string(">==")).map(|_| BinaryOp::GreaterOrEq),
    attempt(string(">=")).map(|_| BinaryOp::GreaterOrEq),
    char('>').map(|_| BinaryOp::Greater),
    attempt(string("<==")).map(|_| BinaryOp::LessOrEq),
    attempt(string("<=")).map(|_| BinaryOp::LessOrEq),
    char('<').map(|_| BinaryOp::Less),
  )).skip(whitespace());
  (addsub(), many((op, addsub()))).map(|(h, t): (Expr, Vec<(BinaryOp, Expr)>)| // todo: to_tree
    t.into_iter().fold(h, |acc, (op, expr)|
      Expr::BinaryOp(op, Box::new(acc), Box::new(expr))
    )
  )
}

fn expr<'a, I>() -> impl Parser<Input = I, Output = Expr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  comparison()
}

fn id<'a, I>() -> impl Parser<Input = I, Output = String> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  let quoted = between(char('"'), char('"'), take_while(|c| c != '"' && c != '\n'))
    .map(|id: &str| id.to_owned());  //todo: to lowercase

  let simple = look_ahead(alpha_num().or(char('_'))).then(|c: char|
    if c.is_numeric() {
      take_while1(|c: char| c.is_alphanumeric() || c == '_').then(|id: &str| //todo: to lowercase
        if id.chars().all(|c| c.is_numeric()) {
          unexpected_any("Expected Id but got number").left()
        } else {
          value(id.to_owned()).right()
        }
      ).left()
    } else {
      recognize(
        skip_many1(attempt(skip_many(char('-')).skip(alpha_num().or(char('_')))))
      ).map(|id: &str| id.to_owned()).right() //todo: to lowercase
    }
  );

  quoted.or(simple).skip(whitespace())
}

fn begin<'a, I>() -> impl Parser<Input = I, Output = String> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  keyword("begin")
    .with(whitespace())
    .with(id())
    .skip(lineend())
}

fn end<'a, I>() -> impl Parser<Input = I, Output = Option<String>> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  keyword("end")
    .with(whitespace())
    .with(optional(id()))
}

fn float_var<'a, I>() -> impl Parser<Input = I, Output = Instr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  keyword("float")
    .with(whitespace())
    .with(id())
    .map(Instr::FloatVar)
    .skip(lineend())
}

fn short_var<'a, I>() -> impl Parser<Input = I, Output = Instr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  keyword("short")
    .with(whitespace())
    .with(id())
    .map(Instr::ShortVar)
    .skip(lineend())
}

fn long_var<'a, I>() -> impl Parser<Input = I, Output = Instr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  keyword("long")
    .with(whitespace())
    .with(id())
    .map(Instr::LongVar)
    .skip(lineend())
}

fn set_to<'a, I>() -> impl Parser<Input = I, Output = Instr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  (
    keyword("set"),
    whitespace(),
    id(),
    optional(char('.').with(whitespace()).with(id())),
    keyword("to"),
    whitespace(),
    expr(),
    lineend(),
  ).map(|(_, _, name, var_name, _, _, expr, _)| match var_name {
    Some(var_name) => Instr::SetTo(Some(name), var_name, expr),
    None => Instr::SetTo(None, name, expr),
  })
}

fn ifelse<'a, I>() -> impl Parser<Input = I, Output = Instr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  // wrap to parser since `first` variable has to be local
  parser(|i| {
    let mut first = true;
    let if_keyword = factory(move || if first {
      first = false;
      keyword("if")
    } else {
      keyword("elseif")
    });
    let cond_block = (
      if_keyword,
      whitespace(),
      expr(),
      lineend(),
      many(instr()),
    ).map(|(_, _, cond, _, instrs)| {
      let _: Vec<Instr> = instrs;
      (cond, instrs)
    });
    let else_block = (
      keyword("else"),
      lineend(),
      many(instr()),
    ).map(|(_, _, instrs)| {
      let _: Vec<Instr> = instrs;
      instrs
    });
    (
      many1(cond_block),
      optional(else_block),
      keyword("endif"),
      lineend(),
    ).map(|(cond_blocks, else_block, _, _)|
      Instr::If(cond_blocks, else_block.unwrap_or_else(Vec::new))
    ).parse_stream(i)
  })
}

fn whileloop<'a, I>() -> impl Parser<Input = I, Output = Instr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  let opaque_instr = parser(|i| instr().parse_stream(i));
  (
    keyword("while"),
    whitespace(),
    expr(),
    lineend(),
    many(opaque_instr),
    keyword("endwhile"),
    lineend(),
  ).map(|(_, _, cond, _, instrs, _, _)|
    Instr::While(cond, instrs)
  )
}

fn proc<'a, I>() -> impl Parser<Input = I, Output = Instr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  fix().then(|(fix, name)| match Func::by_name(&name) {
    Some(proc) => {
      (
        value(fix),
        args(proc.args(), proc.required())
      ).map(move |(fix, args)|
        Instr::Proc(fix, proc, args)
      ).left()
    },
    None => {
      unexpected_any("Unexpected identifier").right()
    },
  }).skip(lineend())
}

fn instr<'a, I>() -> impl Parser<Input = I, Output = Instr> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  // not_followed_by is needed because proc can fail parsing keywords
  // the same could be done wrapping proc into attempt but this screws error messages
  // TODO: parse end without name?
  let not_keyword = not_followed_by(
    choice((
      keyword("elseif").map(|_| "Elseif keyword should not be parsed as Instr"),
      keyword("else").map(|_| "Else keyword should not be parsed as Instr"),
      keyword("endif").map(|_| "Endif keyword should not be parsed as Instr"),
      keyword("endwhile").map(|_| "Endwhile keyword should not be parsed as Instr"),
      keyword("end").map(|_| "End keyword should not be parsed as Instr"),
    )).skip(not_followed_by(alpha_num().or(char('_'))))
  );
  not_keyword.with(choice(( //todo move .skip(lineend()) here?
    float_var(),
    short_var(),
    long_var(),
    set_to(),
    ifelse(),
    whileloop(),
    proc(),
  )))
}

pub fn script<'a, I>() -> impl Parser<Input = I, Output = Script> + 'a
where
  I: RangeStream<Item = char, Range = &'a str> + 'a,
  I::Error: ParseError<I::Item, I::Range, I::Position>,
{
  (
    whitespace(),
    skip_many(lineend()),
    begin(),
    many(instr()),
    end(),
  ).map(|(_, _, name, instrs, _)|
    Script {
      name,
      instrs
    }
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_id_quoted() {
    let (id, _) = id().parse("\" abc def ; 123 +- \"").unwrap();
    assert_eq!(id, " abc def ; 123 +- ");
  }

  #[test]
  fn parse_id_simple_first_letter() {
    let (id, _) = id().parse("a_b-c-1").unwrap();
    assert_eq!(id, "a_b-c-1");
  }

  #[test]
  fn parse_id_simple_first_number() {
    let (id, _) = id().parse("1xyz").unwrap();
    assert_eq!(id, "1xyz");
  }

  #[test]
  fn parse_id_simple_first_number_with_minus() {
    assert!(id().parse("1-1xyz").is_err());
  }

  #[test]
  fn parse_expr() {
    expr().parse("[(2++2)*2]--2>=2/2+-+x-+-GetSquareRoot 2").unwrap(); // TODO: split to several tests
  }

  #[test]
  fn parse_float_var() {
    let (instr, _) = instr().parse("float x \n").unwrap();
    assert_eq!(instr, Instr::FloatVar("x".to_owned()));
  }

  #[test]
  fn parse_short_var() {
    let (instr, _) = instr().parse("short x \n").unwrap();
    assert_eq!(instr, Instr::ShortVar("x".to_owned()));
  }

  #[test]
  fn parse_long_var() {
    let (instr, _) = instr().parse("long x \n").unwrap();
    assert_eq!(instr, Instr::LongVar("x".to_owned()));
  }

  #[test]
  fn parse_set_to() {
    let (instr, _) = instr().parse("set x to 1 \n").unwrap();
    assert_eq!(instr, Instr::SetTo(None, "x".to_owned(), Expr::Long(1)));
  }

  #[test]
  fn parse_set_to_fix() {
    let (instr, _) = instr().parse("set a.x to 1 \n").unwrap();
    assert_eq!(instr, Instr::SetTo(Some("a".to_owned()), "x".to_owned(), Expr::Long(1)));
  }

  #[test]
  fn parse_instr_if() {
    let (instr, _) = instr().parse(
      "\
      if(1) \n\
      endif \n\
      "
    ).unwrap();
    assert_eq!(instr, Instr::If(
      vec![
        (Expr::Brackets(Box::new(Expr::Long(1))), vec![]),
      ],
      vec![]
    ));
  }

  #[test]
  fn parse_instr_if_else() {
    let (instr, _) = instr().parse(
      "\
      if(1) \n\
      else \n\
      endif \n\
      "
    ).unwrap();
    assert_eq!(instr, Instr::If(
      vec![
        (Expr::Brackets(Box::new(Expr::Long(1))), vec![]),
      ],
      vec![]
    ));
  }

  #[test]
  fn parse_instr_if_elseif_else() {
    let (instr, _) = instr().parse(
      "\
      if(1) \n\
      elseif 2 \n\
      else \n\
      endif \n\
      "
    ).unwrap();
    assert_eq!(instr, Instr::If(
      vec![
        (Expr::Brackets(Box::new(Expr::Long(1))), vec![]),
        (Expr::Long(2), vec![]),
      ],
      vec![]
    ));
  }

  #[test]
  fn parse_instr_if_elseif() {
    let (instr, _) = instr().parse(
      "\
      if(1) \n\
        return \n\
      elseif 2 \n\
        short x \n\
      endif \n\
      "
    ).unwrap();
    assert_eq!(instr, Instr::If(
      vec![
        (Expr::Brackets(Box::new(Expr::Long(1))), vec![Instr::Proc(None, Func::Return, vec![])]),
        (Expr::Long(2), vec![Instr::ShortVar("x".to_owned())]),
      ],
      vec![]
    ));
  }

  #[test]
  fn parse_instr_while() {
    let (instr, _) = instr().parse(
      "\
      while(1) \n\
        short x \n\
      endwhile \n\
      "
    ).unwrap();
    assert_eq!(instr, Instr::While(
      Expr::Brackets(Box::new(Expr::Long(1))),
      vec![Instr::ShortVar("x".to_owned())]
    ));
  }

  #[test]
  fn parse_instr_proc() {
    let (instr, _) = instr().parse("StartScript test \n").unwrap();
    assert_eq!(instr, Instr::Proc(
      None,
      Func::StartScript,
      vec![Arg::Id("test".to_owned())]
    ));
  }

  #[test]
  fn parse_instr_proc_fix() {
    let (instr, _) = instr().parse("Player->AddItem gold_001 100 \n").unwrap();
    assert_eq!(instr, Instr::Proc(
      Some("Player".to_owned()),
      Func::AddItem,
      vec![Arg::Id("gold_001".to_owned()), Arg::Expr(Expr::Long(100))]
    ));
  }

  #[test]
  fn parse_script() {
    let (script, _) = script().parse(
      "\
      begin script \n\
        short x \n\
      end script \n\
      "
    ).unwrap();
    assert_eq!(script, Script {
      name: "script".to_owned(),
      instrs: vec![
        Instr::ShortVar("x".to_owned()),
      ],
    });
  }
}
