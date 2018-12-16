#[macro_use]
extern crate nom;

use chrono::{Duration, Utc};
use pathfinding::prelude::{absdiff, astar_bag, Grid};
// use itertools::Itertools;
use ndarray::Array2;
use nom::{alpha, digit};
use regex::Regex;
use std::collections::*;
use std::io::prelude::*;
use tap::TapOps;

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;

    let mut file = File::open("input-day15")?;
    // let mut buf = Vec::new();
    // file.read_to_end(&mut buf)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let (map, units) = parse_input(&buf);

    println!("day15.1 {}", part1(&map, &units));
    // println!("day15.2 {}", part2());

    Ok(())
}

fn parse_input(input: &str) -> (Map, Vec<Unit>) {
    use std::cmp::max;
    let mut width = 0_usize;

    let map: Vec<Tile> = input
        .lines()
        .map(|ln| {
            width = max(width, ln.len());
            ln.chars()
        })
        .flatten()
        .map(|c| match c {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            'G' => Tile::Open,
            'E' => Tile::Open,
            _ => unreachable!(),
        })
        .collect();
    let height = map.len() / width;

    let mut map: Array2<Tile> = Array2::from_shape_vec((height, width), map).unwrap();
    map.swap_axes(0, 1);

    let units: Vec<Unit> = input
        .lines()
        .enumerate()
        .map(|(y, ln)| {
            ln.chars().enumerate().filter_map(move |(x, c)| match c {
                'E' => Some(Unit {
                    utype: UnitType::Elf,
                    coord: (x, y),
                    hitpoints: 200,
                    attack_power: 3,
                }),
                'G' => Some(Unit {
                    utype: UnitType::Goblin,
                    coord: (x, y),
                    hitpoints: 200,
                    attack_power: 3,
                }),
                _ => None,
            })
        })
        .flatten()
        .collect();
    (map, units)
}

type Map = Array2<Tile>;

type Coord = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Debug, PartialEq, Clone)]
struct Unit {
    utype: UnitType,
    coord: Coord,
    hitpoints: usize,
    attack_power: usize,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Open,
}

const COMPASS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn part1(map: &Map, units: &[Unit]) -> usize {
    let mut units = units.to_vec();
    let mut to_process = units.to_vec();

    unimplemented!()
}

fn next_frame(map: &Map, units: &[Unit]) -> Vec<Unit> {
    use std::usize;
    let mut out = units.to_vec();

    let width = map.shape()[0];
    for unit in out.iter_mut() {
        let target = closest_target(&unit, &units, width);
        // TODO: attack logic

        let destinations = {
            let tgts = targets(&unit, &units);
            let mut open_squares: HashSet<Coord> = tgts
                .map(|t| {
                    COMPASS
                        .iter()
                        .map(move |(vx, vy)| (t.coord.0 as isize + vx, t.coord.1 as isize + vy))
                })
                .flatten()
                .filter(|&(x, y)| {
                    x >= 0 && y >= 0 && map.get((x as usize, y as usize)).unwrap() == &Tile::Open
                })
                .map(|(x, y)| (x as usize, y as usize))
                .collect();
            for x in units.iter() {
                open_squares.remove(&x.coord);
            }
            open_squares
        };

        let start = unit.coord;
        let mut grid = create_grid(&map, &units);
        grid.add_vertex(start);
        let mut paths: HashMap<&Coord, Option<Vec<Steps>>> = destinations
            .iter()
            .map(move |goal| {
                (
                    goal,
                    astar_bag(
                        &start,
                        |p| {
                            grid.neighbours(p)
                                .iter()
                                .map(|c| (*c, 1_usize))
                                .collect::<Vec<(Coord, usize)>>()
                        },
                        |p| manhattan_distance(p, goal) / 3,
                        |p| p == goal,
                    )
                    .map(|(steps, _)| steps.into_iter().collect()),
                )
            })
            .collect();

        let (_min_path_len, closest_destinations): (usize, Option<Vec<&Coord>>) =
            paths.iter().fold(
                (usize::MAX, None),
                |(acc_steps, mut acc_dests), (coord, steps)| {
                    if let Some(steps) = steps {
                        let step_len = steps[0].len();
                        if step_len < acc_steps {
                            acc_dests.replace(vec![*coord]);
                        } else if step_len == acc_steps {
                            match acc_dests.as_mut() {
                                Some(v) => v.push(coord),
                                None => {}
                            }
                        }
                    }
                    (acc_steps, acc_dests)
                },
            );
        if closest_destinations.is_none() {
            // TODO: logic for all destinations unreachable
        }
        let destination: &Coord = closest_destinations
            .unwrap()
            .iter()
            .min_by(|(ax, ay), (bx, by)| {
                let av = ay * width + ax;
                let bv = by * width + bx;
                av.cmp(&bv)
            })
            .unwrap();
        let paths_to_destination: Vec<Steps> = paths.remove(&destination).unwrap().unwrap();
        let next_step: Coord = paths_to_destination
            .iter()
            .map(|p| p[0])
            .min_by(|a, b| top_left(a, b, width))
            .unwrap();
        unit.coord = next_step;
    }

    out
}

type Steps = Vec<Coord>;

fn top_left(a: &Coord, b: &Coord, width: usize) -> std::cmp::Ordering {
    let (ax, ay) = a;
    let (bx, by) = b;
    let av = ay * width + ax;
    let bv = by * width + bx;
    av.cmp(&bv)
}

fn manhattan_distance(a: &Coord, b: &Coord) -> usize {
    absdiff(a.0, b.0) + absdiff(a.1, b.1)
}

fn targets_in_range<'a>(unit: &'a Unit, targets: &'a [Unit]) -> impl Iterator<Item = &'a Unit> {
    let search: HashSet<Coord> = COMPASS
        .iter()
        .map(move |(vx, vy)| (unit.coord.0 as isize + vx, unit.coord.1 as isize + vy))
        .filter_map(|(x, y)| {
            if x >= 0 && y >= 0 {
                Some((x as usize, y as usize))
            } else {
                None
            }
        })
        .collect();
    targets.iter().filter(move |t| search.contains(&t.coord))
}

fn closest_target<'a>(unit: &'a Unit, targets: &'a [Unit], width: usize) -> Option<&'a Unit> {
    use std::usize;

    let (_, min_by_hp): (_, Vec<&Unit>) = targets_in_range(&unit, &targets).fold(
        (usize::MAX, Vec::new()),
        |(min_hp, mut mins), x| {
            if x.hitpoints < min_hp {
                mins.clear();
                mins.push(x);
                (x.hitpoints, mins)
            } else if x.hitpoints == min_hp {
                mins.push(x);
                (min_hp, mins)
            } else {
                (min_hp, mins)
            }
        },
    );
    let the_target: Option<&Unit> = {
        min_by_hp.iter().fold(None, |mut acc, tgt| {
            if let Some(min_tgt) = acc {
                let (min_x, min_y) = min_tgt.coord;
                let min_v = min_y * width + min_x;

                let (x, y) = tgt.coord;
                let tgt_v = min_y * width + min_x;

                if tgt_v < min_v {
                    acc.replace(tgt);
                }
            } else {
                acc.replace(tgt);
            }
            acc
        })
    };

    the_target
}

fn create_grid(map: &Map, units: &[Unit]) -> Grid {
    let shape = map.shape();
    let mut grid = Grid::new(shape[0], shape[1]);
    grid.disable_diagonal_mode();

    let unit_locations: HashSet<Coord> = units.iter().map(|u| u.coord).collect();
    for ((x, y), _) in map
        .indexed_iter()
        .filter(|(_, tile)| tile == &&Tile::Open)
        .filter(|((x, y), _)| !unit_locations.contains(&(*x, *y)))
    {
        grid.add_vertex((x, y));
    }

    grid
}

fn targets<'a>(unit: &'a Unit, haystack: &'a [Unit]) -> impl Iterator<Item = &'a Unit> {
    let utype = &unit.utype;
    haystack.iter().filter(move |x| &x.utype != utype)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testtest() {
        assert!(true);
    }
}
