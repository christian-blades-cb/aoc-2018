#[macro_use]
extern crate nom;
extern crate chrono;
extern crate itertools;
extern crate regex;
extern crate tap;
extern crate topological_sort;

use chrono::{Duration, Utc};
use itertools::Itertools;
use nom::{alpha, digit};
use regex::Regex;
use std::io::prelude::*;
use tap::TapOps;

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;

    let mut file = File::open("input-day8")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

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
