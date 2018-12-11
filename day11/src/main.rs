#[macro_use]
extern crate nom;

use chrono::{Duration, Utc};
//use itertools::Itertools;
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

    let input = 9424;

    println!("day10.1 {:?}", part1(input));

    let grid = create_grid(input);

    println!("day10.2 {:?}", part2(&grid));

    Ok(())
}

fn power_level(x: usize, y: usize, serial: usize) -> isize {
    let rack_id = x + 10;
    // println!("rack_id: {}", rack_id);
    let power_level = rack_id * y;
    // println!("power_level: {}", power_level);
    let power_level = power_level + serial;
    // println!("power_level: {}", power_level);
    let power_level = power_level * rack_id;
    // println!("power_level: {}", power_level);
    let pl = format!("{}", power_level);
    let hundreds = &pl[pl.len() - 3..pl.len() - 2];
    // println!("power_level: {} hundreds: {}", power_level, hundreds);
    let hundreds: usize = hundreds.parse().unwrap();
    hundreds as isize - 5_isize
}

fn part1(input: usize) -> (usize, usize) {
    let grid: Vec<Vec<isize>> = (0..300)
        .into_iter()
        .map(|x| {
            (0..300)
                .into_iter()
                .map(|y| power_level(x + 1, y + 1, input))
                .collect::<Vec<isize>>()
        })
        .collect();
    let mut squares: Vec<(usize, usize)> = Vec::with_capacity(297 * 297);
    for x in 0..300 - 3 {
        for y in 0..300 - 3 {
            squares.push((x + 1, y + 1));
        }
    }
    let square_power: Vec<(usize, usize, isize)> = squares
        .iter()
        .cloned()
        .map(|(x, y)| {
            let coords = square_coords(x, y, 3);
            let total_power = coords.into_iter().map(|(x, y)| grid[x - 1][y - 1]).sum();
            (x, y, total_power)
        })
        .collect();
    let most_powerful: (usize, usize, isize) = *square_power
        .iter()
        .max_by(|(_, _, x), (_, _, y)| x.cmp(y))
        .unwrap();
    println!("most_powerful: {:?}", most_powerful);
    (most_powerful.0, most_powerful.1)
}

fn part2(grid: &Vec<Vec<isize>>) -> (usize, usize, usize) {
    let max_square = (1..=300)
        .into_iter()
        .map(|square_size| {
            let lp = largest_power(grid, square_size);
            println!("square_size: {} largest {:?}", square_size, lp);
            let (x, y, power) = lp;
            ((x, y, square_size), power)
        })
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();
    println!("max_square: {:?}", max_square);
    let (coords, _) = max_square;
    coords
}

fn create_grid(input: usize) -> Vec<Vec<isize>> {
    (0..300)
        .into_iter()
        .map(|x| {
            (0..300)
                .into_iter()
                .map(|y| power_level(x + 1, y + 1, input))
                .collect::<Vec<isize>>()
        })
        .collect()
}

fn square_coords(l: usize, h: usize, width: usize) -> Vec<(usize, usize)> {
    let mut coords: Vec<(usize, usize)> = Vec::with_capacity(width * width);
    for x in l..l + width {
        for y in h..h + width {
            coords.push((x, y));
        }
    }
    coords
}

fn largest_power(grid: &Vec<Vec<isize>>, square_size: usize) -> (usize, usize, isize) {
    let cap = 300 - square_size;
    let mut squares: Vec<(usize, usize)> = Vec::with_capacity(cap * cap);
    for x in 0..cap {
        for y in 0..cap {
            squares.push((x + 1, y + 1));
        }
    }
    let square_power: Vec<(usize, usize, isize)> = squares
        .iter()
        .cloned()
        .map(|(x, y)| {
            let coords = square_coords(x, y, square_size);
            let total_power = coords.into_iter().map(|(x, y)| grid[x - 1][y - 1]).sum();
            (x, y, total_power)
        })
        .collect();
    if square_power.len() == 1 {
        return square_power[0];
    }
    let most_powerful: (usize, usize, isize) = *square_power
        .iter()
        .max_by(|(_, _, x), (_, _, y)| x.cmp(y))
        .unwrap();
    // println!("most_powerful: {:?}", most_powerful);
    most_powerful
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(4, power_level(3, 5, 8));
        assert_eq!(-5, power_level(122, 79, 57));
        assert_eq!(0, power_level(217, 196, 39));
        assert_eq!(4, power_level(101, 153, 71));
    }
}
