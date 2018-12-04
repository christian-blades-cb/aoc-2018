#[macro_use]
extern crate nom;
extern crate regex;
extern crate tap;

use nom::types::CompleteStr;
use nom::{alpha, digit};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use tap::TapOps;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input-day4")?;
    // let mut buf = Vec::new();
    // file.read_to_end(&mut buf)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    println!("{}", buf);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        assert!(true);
    }
}
