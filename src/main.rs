#![feature(generic_associated_types)]

use driver::{Drive, Driver, DriverOptions};
use inkwell::context::Context;
use std::io::{stdin, stdout};

pub mod ast;
pub mod codegen;
pub mod driver;
pub mod environment;
pub mod lexer;
pub mod option_ext;
pub mod parser;
mod test_utilities;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author)]
struct Arguments {
    #[clap(long)]
    print_ir: Option<bool>,

    #[clap(long)]
    print_parse: Option<bool>,
}

fn main() -> Result<(), std::io::Error> {
    let mut args = Arguments::parse();
    if args.print_ir.is_none() {
        args.print_ir = Some(false)
    }
    if args.print_parse.is_none() {
        args.print_parse = Some(false)
    }

    let context = Context::create();
    let mut driver =
        Driver::new(Box::new(stdin()), Box::new(stdout()), &context).with_options(DriverOptions {
            print_ir: args.print_ir.unwrap_or(false),
            print_parses: args.print_ir.unwrap_or(false),
        });

    driver.run()?;
    driver.dump_ir()?;
    Ok(())
}
