#[macro_use]
extern crate nom;
extern crate chrono;
extern crate regex;
extern crate tap;

use chrono::{DateTime, Duration, Utc};
use nom::types::CompleteStr;
use nom::{alpha, digit};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use tap::TapOps;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input-day6")?;
    // let mut buf: Vec<u8> = Vec::new();
    // file.read_to_end(&mut buf)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    println!("Hello, world!");
    Ok(())
}
