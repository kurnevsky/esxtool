#[macro_use]
extern crate bitflags;
extern crate byteorder;
#[macro_use]
extern crate clap;
extern crate combine;
extern crate encoding;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate phf;
extern crate seek_bufread;

#[macro_use]
mod binary;
mod primitive;
mod esx;
mod stats;
mod trace;
mod mwscript;

#[cfg(test)]
mod samples;

use std::fs::File;
use std::io::{BufWriter, Result};

use clap::{App, AppSettings, Arg, SubCommand};
use encoding::all::WINDOWS_1252;
use seek_bufread::BufReader;

use crate::binary::Binary;
use crate::esx::Esx;

fn main() -> Result<()> {
  env_logger::init();

  let input_arg = Arg::with_name("input")
    .long("input")
    .short("i")
    .help("Input file to process")
    .value_name("FILE")
    .required(true);
  let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!("\n"))
    .about(crate_description!())
    .global_setting(AppSettings::ColoredHelp)
    .setting(AppSettings::SubcommandRequired)
    .subcommand(SubCommand::with_name("stats")
                .about("Print the number of each record")
                .arg(input_arg.clone()))
    .subcommand(SubCommand::with_name("trace")
                .about("Print all records with their contents")
                .arg(input_arg.clone())
                .arg(Arg::with_name("scripts")
                     .long("scripts")
                     .short("s")
                     .help("Show scripts")))
    .subcommand(SubCommand::with_name("parse-scripts")
                .about("Parse all scripts")
                .arg(input_arg.clone()))
    .subcommand(SubCommand::with_name("noop")
                .about("Read the file and optionally write it")
                .arg(input_arg)
                .arg(Arg::with_name("output")
                     .long("output")
                     .short("o")
                     .help("Output file")
                     .value_name("FILE")))
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("stats") {
    let input = matches.value_of("input").expect("Input is required");

    let file = File::open(input)?;
    let mut file = BufReader::new(file);
    let esx = Esx::read(&mut file, WINDOWS_1252)?;

    stats::stats(&esx);
  } else if let Some(matches) = matches.subcommand_matches("trace") {
    let input = matches.value_of("input").expect("Input is required");
    let scripts = matches.is_present("scripts");

    let file = File::open(input)?;
    let mut file = BufReader::new(file);
    let esx = Esx::read(&mut file, WINDOWS_1252)?;

    trace::trace(&esx, scripts);
  } else if let Some(matches) = matches.subcommand_matches("parse-scripts") {
    use combine::Parser;
    use combine::stream::state::State;

    use crate::esx::record::Record;
    use crate::esx::record::scpt::sub_record::ScptSubRecord;

    let input = matches.value_of("input").expect("Input is required");

    let file = File::open(input)?;
    let mut file = BufReader::new(file);
    let esx = Esx::read(&mut file, WINDOWS_1252)?;

    let mut invalid_count = 0u32;
    for record in &esx.records {
      if let Record::Scpt(scpt) = record {
        let schd = scpt.sub_records.iter().find_map(|sub_record| match sub_record {
          ScptSubRecord::Schd(schd) => Some(schd),
          _ => None,
        }).expect("Failed to get script header");
        let sctx = scpt.sub_records.iter().find_map(|sub_record| match sub_record {
          ScptSubRecord::Sctx(sctx) => Some(sctx),
          _ => None,
        }).expect("Failed to get script text");
        if let Err(e) = mwscript::parser::script().easy_parse(State::new(&sctx.text as &str)).map(|(script, _)| script) {
          println!("Error while parsing {}:\n{}", schd.name, e);
          invalid_count += 1;
        }
      }
    }

    println!("Total invalid scripts count: {}", invalid_count);
  } else if let Some(matches) = matches.subcommand_matches("noop") {
    let input = matches.value_of("input").expect("Input is required");
    let output = matches.value_of("output");

    let file = File::open(input)?;
    let mut file = BufReader::new(file);
    let esx = Esx::read(&mut file, WINDOWS_1252)?;

    if let Some(output) = output {
      let file = File::create(&output)?;
      let mut file = BufWriter::new(file);
      esx.write(&mut file, WINDOWS_1252)?;
    }
  }

  Ok(())
}
