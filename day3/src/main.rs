#[macro_use]
extern crate nom;
extern crate regex;
extern crate tap;

use nom::types::CompleteStr;
use nom::{alpha, digit, space};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use tap::TapOps;

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
    // Max(0, Min(XA2, XB2) - Max(XA1, XB1)) * Max(0, Min(YA2, YB2) - Max(YA1, YB1))

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

fn collision_dumb(l: &Rect, r: &Rect) -> bool {
    use std::cmp::{max, min};
    let mut grid: HashSet<usize> = HashSet::new();

    let this = l;
    for x in this.x..this.x + this.width {
        for y in this.y..this.y + this.height {
            grid.insert(coord_1d(x, y));
        }
    }
    let this = r;
    for x in this.x..this.x + this.width {
        for y in this.y..this.y + this.height {
            let coord = coord_1d(x, y);
            if grid.contains(&coord) {
                return true;
            }
        }
    }

    false
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

fn area_of_intersection(l: &Rect, r: &Rect) -> usize {
    use std::cmp::{max, min};
    max(0, min(l.x + l.width, r.x + r.width) - max(l.x, r.x))
        * max(0, min(l.y + l.height, r.y + r.height) - max(l.y, r.y))
}

fn collision(rect1: &Rect, rect2: &Rect) -> bool {
    // rect1.x < rect2.x + rect2.width
    //     && rect1.x + rect1.width > rect2.x
    //     && rect1.y < rect2.y + rect2.height
    //     && rect1.y + rect1.height > rect2.y

    // rect1.x < rect2.Right && RectA.Right > RectB.Left &&
    //  RectA.Top > RectB.Bottom && RectA.Bottom < RectB.Top
    rect1.x < rect2.x + rect2.width
        && rect1.x + rect1.width > rect2.x
        && rect1.y > rect2.y + rect2.height
        && rect1.y + rect1.height < rect2.y
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

    fn test_hello() {
        assert!(true);
    }
}
