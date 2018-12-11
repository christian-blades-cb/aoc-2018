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

    let mut file = File::open("input-day11")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let input: usize = buf.parse().unwrap();

    let grid = create_grid(input);
    println!("day10.1 {:?}", part1(&grid));
    println!("day10.2 {:?}", part2(&grid));

    Ok(())
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

fn power_level(x: usize, y: usize, serial: usize) -> isize {
    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + serial;
    let power_level = power_level * rack_id;
    let pl = format!("{}", power_level);
    let hundreds = &pl[pl.len() - 3..pl.len() - 2];
    let hundreds: usize = hundreds.parse().unwrap();
    hundreds as isize - 5_isize
}

fn part1(grid: &Vec<Vec<isize>>) -> (usize, usize) {
    let (x, y, _) = largest_power(grid, 3);
    (x, y)
}

fn part2(grid: &Vec<Vec<isize>>) -> (usize, usize, usize) {
    let max_square = (1..300)
        .into_iter()
        .map(|square_size| {
            let lp = largest_power(grid, square_size);
            println!("square_size: {} largest {:?}", square_size, lp);
            let (x, y, power) = lp;
            ((x, y, square_size), power)
        })
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();

    let full_sum: isize = grid.iter().map(|col| col.iter().sum::<isize>()).sum();
    println!("square_size: 300 largest (1, 1, {})", full_sum);

    if full_sum > max_square.1 {
        (1, 1, 300)
    } else {
        let (coords, _) = max_square;
        coords
    }
}

fn square_coords(l: usize, h: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
    use itertools::Itertools;

    (l..l + width).cartesian_product(h..h + width)
}

fn largest_power(grid: &Vec<Vec<isize>>, square_size: usize) -> (usize, usize, isize) {
    use itertools::Itertools;

    let cap = 300 - square_size;
    let squares = (0..cap)
        .cartesian_product(0..cap)
        .map(|(x, y)| (x + 1, y + 1));
    let square_power = squares.map(|(x, y)| {
        let coords = square_coords(x, y, square_size);
        let total_power: isize = coords.map(|(x, y)| grid[x - 1][y - 1]).sum();
        (x, y, total_power)
    });

    square_power
        .max_by(|(_, _, x): &(usize, usize, isize), (_, _, y)| x.cmp(&y))
        .unwrap()
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

    #[test]
    fn test_part1() {
        let grid = create_grid(18);
        assert_eq!((33, 45), part1(&grid));
    }
}
