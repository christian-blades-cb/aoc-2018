extern crate itertools;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

type Coord = (isize, isize);

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input-day6")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let coords: Vec<Coord> = buf.lines().map(parse_line).collect();

    println!("day6.1 {}", part1(&coords, 300));
    println!("day6.2 {}", part2(&coords, 10000));

    Ok(())
}

fn part1(points: &[Coord], box_size: isize) -> usize {
    use itertools::Itertools;
    use std::sync::{Mutex, MutexGuard};
    use std::usize;

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

fn part2(pts: &[Coord], threshold: usize) -> usize {
    use itertools::Itertools;
    use std::cmp::{max, min};
    use std::{isize, usize};

    let (min_x, min_y, max_x, max_y): (isize, isize, isize, isize) = pts.iter().fold(
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
        |(min_x, min_y, max_x, max_y), (x, y)| {
            let min_x = min(x, &min_x);
            let min_y = min(y, &min_y);
            let max_x = max(x, &max_x);
            let max_y = max(y, &max_y);
            (*min_x, *min_y, *max_x, *max_y)
        },
    );
    let bounding_mod = (threshold / pts.len() + 1) as isize;
    let min_x = min_x - bounding_mod;
    let min_y = min_y - bounding_mod;
    let max_x = max_x + bounding_mod;
    let max_y = max_y + bounding_mod;
    let field = (min_x..=max_x)
        .into_iter()
        .cartesian_product((min_y..=max_y).into_iter())
        .map(|coord| {
            let distsum: usize = pts.iter().map(|p| manhattan_distance(p, &coord)).sum();
            (coord, distsum)
        });
    let less_than_thresh: usize = field
        .map(|(_, distsum)| if distsum < threshold { 1 } else { 0 })
        .sum();
    less_than_thresh
}

fn contains_infinite(pts: &[Coord], box_size: isize) -> bool {
    pts.iter()
        .any(|(x, y)| x.abs() == box_size || y.abs() == box_size)
}

#[inline]
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
        assert_eq!(17, part1(&coords, 30));
    }

    #[test]
    fn test_part2() {
        let buf = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let coords: Vec<Coord> = buf.lines().map(parse_line).collect();
        assert_eq!(16, part2(&coords, 32));
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
