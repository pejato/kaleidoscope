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

fn main() -> Result<(), std::io::Error> {
    let options = DriverOptions::parse();
    let context = Context::create();
    let mut driver =
        Driver::new(Box::new(stdin()), Box::new(stdout()), &context).with_options(options);

    driver.run()?;
    driver.dump_ir()?;
    Ok(())
}
