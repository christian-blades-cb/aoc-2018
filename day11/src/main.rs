#[macro_use]
extern crate nom;

use chrono::{Duration, Utc};
use itertools::Itertools;
use nom::{alpha, digit};
use regex::Regex;
use std::collections::*;
use std::io::prelude::*;
use tap::TapOps;

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;

    let mut file = File::open("input-day10")?;
    // let mut buf = Vec::new();
    // file.read_to_end(&mut buf)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    println!("day10.1 {}", part1());
    // println!("day10.2 {}", part2());

    Ok(())
}

fn part1() -> usize {
    unimplemented!()
}

fn part2() -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        assert!(true);
    }
}