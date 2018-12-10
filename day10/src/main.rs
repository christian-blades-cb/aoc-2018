#[macro_use]
extern crate nom;

use itertools::Itertools;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;

    let mut file = File::open("input-day10")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let points: Vec<Point> = buf.lines().map(|ln| parse_input(ln).unwrap().1).collect();
    part1(&points);

    Ok(())
}

fn find_min_ts(points: &[Point], t0: usize, t1: usize) -> usize {
    (t0..=t1)
        .into_iter()
        .map(|t| {
            let pos: Vec<Position> = points.iter().map(|p| p.at_time(t)).collect();
            let (_, _, width, height) = canvas(&pos);
            (t, width + height)
        })
        .min_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap()
        .0
}

fn part1(points: &[Point]) {
    use image::{ImageBuffer, Luma};

    let min_ts = find_min_ts(points, 10000, 50000);
    println!("min_ts: {}", min_ts);
    let t = min_ts;

    let current_pos: Vec<Position> = points.iter().map(|p| p.at_time(t)).collect();

    let (offset_x, offset_y, width, height) = canvas(&current_pos);
    println!("width: {}, height: {}", width, height);

    let mut img = ImageBuffer::new(width as u32 + 1, height as u32 + 1);

    let px = Luma([255u8]);
    for Position(x, y) in current_pos.iter() {
        img.put_pixel((x + offset_x) as u32, (y + offset_y) as u32, px);
    }

    let filename = format!("day10pt1_ts{}.png", t);
    println!("saving {}", filename);
    img.save(filename).unwrap();
}

// offset_x, offset_y
fn canvas(positions: &[Position]) -> (isize, isize, usize, usize) {
    use itertools::MinMaxResult;
    let (x_offset, width) = match positions.iter().map(|p| p.0).minmax() {
        MinMaxResult::NoElements => unreachable!(),
        MinMaxResult::OneElement(x) => {
            if x < 0 {
                (x.abs(), 0)
            } else {
                (0 - x, 0)
            }
        }
        MinMaxResult::MinMax(x, y) => {
            let off = if x < 0 { x.abs() } else { 0 - x };
            let width = y + off;
            (off, width)
        }
    };
    let (y_offset, height) = match positions.iter().map(|p| p.0).minmax() {
        MinMaxResult::NoElements => unreachable!(),
        MinMaxResult::OneElement(x) => {
            if x < 0 {
                (x.abs(), 0)
            } else {
                (0 - x, 0)
            }
        }
        MinMaxResult::MinMax(x, y) => {
            let off = if x < 0 { x.abs() } else { 0 - x };
            let height = y + off;
            (off, height)
        }
    };
    (x_offset, y_offset, width as usize, height as usize)
}

#[derive(Debug, Clone)]
struct Point {
    position: Position,
    velocity: Velocity,
}

impl Point {
    fn at_time(&self, t: usize) -> Position {
        let mod_x = self.velocity.0 * t as isize;
        let mod_y = self.velocity.1 * t as isize;
        Position(self.position.0 + mod_x, self.position.1 + mod_y)
    }
}
#[derive(Debug, Clone)]
struct Position(isize, isize);
#[derive(Debug, Clone)]
struct Velocity(isize, isize);

named!(parse_input<&str, Point>,
       do_parse!(
           tag!("position=<") >>
               pos_x: take!(6) >>
               tag!(", ") >>
               pos_y: take!(6) >>
               tag!("> velocity=<") >>
               vel_x: take!(2) >>
               tag!(", ") >>
               vel_y: take!(2) >>
               (Point{
                   position: Position(pos_x.trim().parse().unwrap(), pos_y.trim().parse().unwrap()),
                   velocity: Velocity(vel_x.trim().parse().unwrap(), vel_y.trim().parse().unwrap()),
               })));
