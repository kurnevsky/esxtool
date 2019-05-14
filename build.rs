extern crate phf_codegen;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use phf_codegen::Map;
use regex::{CaptureMatches, Regex};

fn build_map(enum_name: &str, names: CaptureMatches) -> Map<String> {
  let mut map = Map::new();
  for cap in names {
    let name = &cap["name"];
    map.entry(
      name.to_lowercase(),
      &format!("{}::{}", enum_name, name)
    );
  }
  map
}

fn main() {
  let out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR");
  let path = Path::new(&out_dir).join("codegen.rs");
  let mut file = BufWriter::new(File::create(&path).expect("Failed to create codegen.rs"));

  let mut funcs_file = BufReader::new(File::open("src/mwscript/funcs.rs").expect("Failed to open funcs.rs"));
  let mut contents = String::new();
  funcs_file.read_to_string(&mut contents).expect("Failed to read funcs.rs");

  let names_re = Regex::new(r"\n\s*(?P<name>[[:alnum:]]+).*").expect("Failed to build names regex");
  let funcs_re = Regex::new(r"mwfuncs! \{[^}]*\}").expect("Failed to build mwfuncs regex");
  let funcs_str = funcs_re.find(&contents).expect("Failed to find mwfuncs in funcs.rs").as_str();
  let funcs_map = build_map("Func", names_re.captures_iter(&funcs_str));
  write!(&mut file, "static FUNCS: phf::Map<&'static str, Func> = ").expect("Failed to write codegen.rs");
  funcs_map.build(&mut file).expect("Failed to write codegen.rs");
  write!(&mut file, ";\n").expect("Failed to write codegen.rs");
}
