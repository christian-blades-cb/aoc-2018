#[macro_use]
extern crate nom;

use nom::digit;
use nom::types::CompleteStr;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input-day3")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let boxes: Vec<Rect> = buf
        .lines()
        .map(|l| {
            let (_, bo) = parse_box(l.into()).unwrap();
            bo
        })
        .collect();

    println!("day3.1 {}", part1(&boxes));
    println!("day3.2 {}", part2(&boxes));

    Ok(())
}

#[inline]
fn coord_1d(x: usize, y: usize) -> usize {
    let width = 1000;
    width * y + x
}

fn part1(boxes: &[Rect]) -> usize {
    let mut grid: Vec<usize> = vec![0usize; 1000 * 1000];
    for r in boxes {
        for x in r.x..r.x + r.width {
            for y in r.y..r.y + r.height {
                grid[coord_1d(x, y)] += 1;
            }
        }
    }
    grid.iter().map(|x| if *x > 1 { 1 } else { 0 }).sum()
}

fn part2(boxes: &[Rect]) -> usize {
    let all_ids: HashSet<usize> = boxes.iter().map(|b| b.id).collect();
    let mut collided: HashSet<usize> = HashSet::new();
    let mut claimed: HashMap<usize, usize> = HashMap::new();
    for bx in boxes {
        for x in bx.x..bx.x + bx.width {
            for y in bx.y..bx.y + bx.height {
                if let Some(collided_with) = claimed.insert(coord_1d(x, y), bx.id) {
                    collided.insert(bx.id);
                    collided.insert(collided_with);
                }
            }
        }
    }

    let mut uncollided = all_ids.difference(&collided);
    uncollided.next().map(|x| *x).unwrap()
}

named!(parse_box<CompleteStr, Rect>,
       do_parse!(
           tag!("#") >>
               id: digit >>
               tag!(" @ ") >>
               x: digit >>
               tag!(",") >>
               y: digit >>
               tag!(": ") >>
               width: digit >>
               tag!("x") >>
               height: digit >>
               (
                   Rect{
                       id: id.parse().unwrap(),
                       x: x.parse().unwrap(),
                       y: y.parse().unwrap(),
                       width: width.parse().unwrap(),
                       height: height.parse().unwrap(),
                   }
               )
       ));

#[derive(Clone, Debug)]
struct Rect {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_boxes() -> Vec<Rect> {
        let buf = include_str!("../../input-day3");
        let boxes: Vec<Rect> = buf
            .lines()
            .map(|l| {
                let (_, bo) = parse_box(l.into()).unwrap();
                bo
            })
            .collect();
        boxes
    }

    #[test]
    fn test_part1() {
        let boxes = get_boxes();
        assert_eq!(104439, part1(&boxes));
    }

    #[test]
    fn test_part2() {
        let boxes = get_boxes();
        assert_eq!(701, part2(&boxes));
    }
}
