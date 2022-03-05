#![feature(generic_associated_types)]
#![feature(stdio_locked)]

use driver::{Drive, Driver, DriverOptions};
use inkwell::context::Context;
use std::io::{stdin, stdout};

mod ast;
mod codegen;
mod driver;
mod environment;
mod lexer;
mod library;
mod option_ext;
mod parser;
mod test_utilities;

use clap::Parser;

fn main() -> Result<(), std::io::Error> {
    let options = DriverOptions::parse();
    let context = Context::create();
    let mut driver =
        Driver::new(Box::new(stdin()), Box::new(stdout()), &context).with_options(options);

    driver.run()?;
    driver.dump_ir()?;
    Ok(())
}
