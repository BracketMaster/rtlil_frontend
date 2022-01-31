// Copyright (c) 2020 xhe
// Copyright (c) 2020 Yehowshua

use rtlil_frontend::{lexer::Lexer, parser::Parser, dumper::Dumper, visitor::Visit};
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();
	match args.len() {
		2 => {
			let input = fs::read_to_string(&args[1])?;
			let lx = Lexer::new(input.chars());
            let mut pr = Parser::new();
			let res = pr.parse(lx)?;
			res.visit(&mut Dumper::new())?;
			// dbg!(res);
			
			// let ident = res.modules().get(0).unwrap().ident();
			// dbg!(ident);
			// println!("Finished");
		}
		_ => println!("main [input]"),
	}
	Ok(())
}
