#[macro_use]
extern crate nom;
extern crate chrono;
extern crate itertools;
extern crate regex;
extern crate tap;
extern crate topological_sort;

use chrono::{DateTime, Duration, Utc};
use itertools::Itertools;
use nom::types::CompleteStr;
use nom::{alpha, digit};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use tap::TapOps;
use topological_sort::TopologicalSort;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input-day7")?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    // let mut buf = String::new();
    // file.read_to_string(&mut buf)?;

    let input: Vec<(u8, u8)> = buf
        .split(|x| *x == b'\n')
        .map(|ln| parse_step(ln).unwrap().1)
        .collect();
    println!("{:?}", input);

    let order = part1(&input);
    let mut outstring = String::new();
    for x in order {
        outstring.push((*x).into());
    }
    println!("day7.1 {}", outstring);

    Ok(())
}

fn print_vec_as_string(input: &[&u8]) -> String {
    let mut outstring = String::new();
    for x in input {
        outstring.push((**x).into());
    }
    outstring
}

fn part1(input: &[(u8, u8)]) -> Vec<&u8> {
    use std::collections::HashSet;

    let mut popped_prec: HashSet<&u8> = HashSet::new();
    let all_the_things: HashSet<&u8> = input.iter().fold(HashSet::new(), |acc, (a, b)| {
        acc.tap(|accum| {
            accum.insert(a);
            accum.insert(b);
        })
    });

    let mut order: Vec<&u8> = Vec::new();
    loop {
        let mut topology: TopologicalSort<&u8> = TopologicalSort::new();
        for (prec, succ) in input {
            if popped_prec.contains(prec) {
                continue;
            }
            topology.add_dependency(prec, succ);
        }

        let mut to_order = topology.pop_all();
        println!("to_order: {:?}", print_vec_as_string(&to_order));
        if to_order.len() == 0 {
            break;
        }

        to_order.sort();
        let prec = to_order[0];
        order.push(prec);
        popped_prec.insert(prec);

        println!("order: {:?}", print_vec_as_string(&order));
    }
    for succ in all_the_things.difference(&popped_prec) {
        order.push(succ);
    }
    order
}

named!(parse_step<&[u8], (u8, u8)>,
       do_parse!(
           tag!("Step ") >>
               prec: take!(1) >>
               tag!(" must be finished before step ") >>
               succ: take!(1) >>
               ((prec[0], succ[0]))));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let buf = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
            .as_bytes();
        let input: Vec<(u8, u8)> = buf
            .split(|x| *x == b'\n')
            .map(|ln| parse_step(ln).unwrap().1)
            .collect();
        let order = part1(&input);
        let mut outstring = String::new();
        for x in order {
            outstring.push((*x).into());
        }
        assert_eq!("CABDFE", outstring);
    }
}
