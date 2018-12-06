#[macro_use]
extern crate nom;
extern crate chrono;
extern crate itertools;
extern crate rayon;
extern crate regex;
extern crate tap;

use chrono::{DateTime, Duration, Utc};
use nom::types::CompleteStr;
use nom::{alpha, digit};
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use tap::TapOps;

type Coord = (isize, isize);

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input-day6")?;
    // let mut buf: Vec<u8> = Vec::new();
    // file.read_to_end(&mut buf)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let coords: Vec<Coord> = buf.lines().map(parse_line).collect();
    let largest_area = part1(&coords, 2000);

    println!("day6.1 {}", largest_area); // not 556399 too high

    Ok(())
}

#[inline]
fn one_dim_to_two_dim(n: isize, width: isize) -> Coord {
    let width = width.abs();
    let y = n / width;
    let x = n % width;
    (x, y)
}

fn part1(points: &[Coord], box_size: isize) -> usize {
    use itertools::{max, min, Itertools};
    use std::sync::{Mutex, MutexGuard};
    use std::{isize, usize};

    let grid = (0 - box_size..box_size + 1)
        .into_iter()
        .cartesian_product((0 - box_size..box_size + 1).into_iter())
        .filter_map(|this| {
            let pt_dist: Vec<(&Coord, usize)> = points
                .iter()
                .map(|pt| (pt, manhattan_distance(pt, &this)))
                .collect();
            let min_dist = pt_dist.iter().map(|(_, dist)| dist).min().unwrap();
            let mut min_coords = pt_dist.iter().filter(|(_, dist)| dist == min_dist);
            let closest = min_coords.next().unwrap();
            if let Some(_) = min_coords.next() {
                None
            } else {
                Some((this, closest.0))
            }
        });

    let acc = Mutex::new(HashMap::new());
    grid.for_each(|(coord, closest)| {
        let mut guard: MutexGuard<HashMap<&Coord, Vec<Coord>>> = acc.lock().unwrap();
        guard
            .entry(closest)
            .and_modify(|e| e.push(coord))
            .or_insert(vec![coord]);
    });
    let grouped: HashMap<&Coord, Vec<Coord>> = acc.into_inner().unwrap();

    let infinites: HashSet<&Coord> = grouped
        .iter()
        .filter(|(_coord, pts)| contains_infinite(pts, box_size))
        .map(|(coord, _)| *coord)
        .collect(); // coords whose area touches the boundary

    let areas: HashMap<&Coord, usize> = grouped
        .iter()
        .map(|(coord, pts)| (*coord, pts.len()))
        .collect();
    let max_area: usize = areas
        .iter()
        .filter_map(|(coord, area)| {
            if infinites.contains(coord) {
                None
            } else {
                Some(*area)
            }
        })
        .max()
        .unwrap();

    max_area
}

fn contains_infinite(pts: &[Coord], box_size: isize) -> bool {
    pts.iter()
        .any(|(x, y)| x.abs() == box_size || y.abs() == box_size)
}

fn manhattan_distance(lhs: &Coord, rhs: &Coord) -> usize {
    use std::isize;
    let (l_x, l_y) = lhs;
    let (r_x, r_y) = rhs;

    let x = (*l_x as isize - *r_x as isize).abs();
    let y = (*l_y as isize - *r_y as isize).abs();
    (x + y) as usize
}

fn parse_line(ln: &str) -> Coord {
    let mut sp = ln.split(", ").take(2).map(|x| x.parse::<isize>().unwrap());
    let a = sp.next().unwrap();
    let b = sp.next().unwrap();
    (a, b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let buf = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let coords: Vec<Coord> = buf.lines().map(parse_line).collect();
        println!("coords: {:?}", coords);
        assert_eq!(17, part1(&coords, 30));
    }

    #[test]
    fn test_infinite() {
        assert_eq!(true, contains_infinite(&vec![(-20, 0)], 20));
        assert_eq!(true, contains_infinite(&vec![(0, -20)], 20));
        assert_eq!(true, contains_infinite(&vec![(20, 0)], 20));
        assert_eq!(true, contains_infinite(&vec![(0, 20)], 20));
        assert_eq!(false, contains_infinite(&vec![(0, 0)], 20));
    }
}
