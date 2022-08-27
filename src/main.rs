extern crate pest;
#[macro_use]
extern crate pest_derive;

mod args;
mod ast;

use pest::Parser;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser)]
#[grammar = "lang.pest"]
pub struct LangParser;

fn main() -> Result<(), std::io::Error>{
	let args = args::get_args();
	println!("File {}", &args.name);

	let mut file = File::open(args.name)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	match LangParser::parse(Rule::main, contents.as_str()) {
		Ok(a) => println!("{}", a),
		Err(a) => println!("{}", a),
	}

	Ok(())
}
