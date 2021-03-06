extern crate csv;

use std::io;
use std::error::Error;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::process;

pub fn parse() -> Result<(), Box<dyn Error>>{
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        dbg!(record);
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        dbg!(record);
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>>{
    match  env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path)
    }
}
