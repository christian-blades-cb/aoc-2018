extern crate tap;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use tap::TapOps;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input-day2")?;
    // let mut buf = Vec::new();
    // file.read_to_end(&mut buf)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let boxids: Vec<String> = buf.lines().map(|s| s.into()).collect();

    let checksum = part1(&boxids);
    println!("day2.1 {}", checksum);

    let common = part2(&boxids);
    println!("day2.2 {:?}", common);

    Ok(())
}

fn part1(codes: &[String]) -> usize {
    let mut counts: HashMap<String, CharCounts> = HashMap::new();
    for line in codes.iter() {
        let char_counts = line.chars().fold(CharCounts::new(), |acc, c| {
            acc.tap(|a| *a.entry(c).and_modify(|e| *e += 1).or_insert(1))
        });
        counts.insert(line.clone(), char_counts);
    }

    let two_counts: usize = counts
        .iter()
        .map(|(_, hm)| if has_two(hm) { 1 } else { 0 })
        .sum();
    let three_counts: usize = counts
        .iter()
        .map(|(_, hm)| if has_three(hm) { 1 } else { 0 })
        .sum();
    two_counts * three_counts
}

fn part2(codes: &[String]) -> Option<String> {
    let mut boxids = codes.to_vec();
    loop {
        if boxids.len() == 0 {
            break;
        }
        let this = boxids.pop().unwrap();
        for other in boxids.iter() {
            if char_diff(&this, other) == 1 {
                let common = common_letters(&this, other);
                return Some(common);
            }
        }
    }
    None
}

fn common_letters(lhs: &str, rhs: &str) -> String {
    lhs.chars()
        .zip(rhs.chars())
        .fold(String::new(), |acc, (l, r)| {
            if l == r {
                acc.tap(|a| a.push(l))
            } else {
                acc
            }
        })
}

#[inline]
fn char_diff(lhs: &str, rhs: &str) -> usize {
    lhs.chars()
        .zip(rhs.chars())
        .map(|(l, r)| if l == r { 0 } else { 1 })
        .sum()
}

#[inline]
fn has_two(hm: &CharCounts) -> bool {
    hm.iter().any(|(_, count)| *count == 2)
}

#[inline]
fn has_three(hm: &CharCounts) -> bool {
    hm.iter().any(|(_, count)| *count == 3)
}

type CharCounts = HashMap<char, usize>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day1() {
        let buf = include_str!("../../input-day2");
        let codes: Vec<String> = buf.lines().map(|l| l.into()).collect();

        assert_eq!(4693, part1(&codes));
    }

    #[test]
    fn test_day2() {
        let buf = include_str!("../../input-day2");
        let codes: Vec<String> = buf.lines().map(|l| l.into()).collect();

        let expected = "pebjqsalrdnckzfihvtxysomg";
        assert_eq!(Some(expected.into()), part2(&codes));
    }
}
