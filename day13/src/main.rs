use ndarray::prelude::*;
use std::collections::*;
use std::io::prelude::*;

const WIDTH: usize = 150;
// const WIDTH: usize = 7;
const UP: Velocity = (0, -1);
const DOWN: Velocity = (0, 1);
const RIGHT: Velocity = (1, 0);
const LEFT: Velocity = (-1, 0);

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;

    let mut file = File::open("input-day13")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let (grid, carts) = parse_input(&buf);
    println!("day13.1 {:?}", part1(&carts, &grid));
    println!("day13.2 {:?}", part2(&carts, &grid));

    Ok(())
}

fn part2(carts: &[Cart], grid: &Grid) -> Coord {
    let mut carts = carts.to_vec();

    loop {
        carts.sort_by(|Cart((a_x, a_y), _, _), Cart((b_x, b_y), _, _)| {
            let a = a_y * WIDTH + a_x;
            let b = b_y * WIDTH + b_x;
            a.cmp(&b)
        });

        let carts_n = carts.len();
        let mut crashed: HashSet<usize> = HashSet::new();
        let mut locations: Vec<Coord> = carts
            .iter()
            .map(|Cart(coord, _, _)| coord.clone())
            .collect();

        if carts_n == 1 {
            let cart = &mut carts[0];
            return cart.0;
        }

        for (i, cart) in carts.iter_mut().enumerate() {
            if crashed.contains(&i) {
                continue;
            }

            cart.next_frame(grid);
            if let Some(other_idx) = locations
                .iter()
                .enumerate()
                .position(|(idx, c)| c == &cart.0 && !crashed.contains(&idx))
            {
                crashed.insert(i);
                crashed.insert(other_idx);
                // println!("crashed at {:?}, carts {} & {}", cart.0, i, other_idx);
            }
            locations[i] = cart.0;
        }

        carts = carts
            .iter()
            .enumerate()
            .filter_map(|(i, ct)| {
                if crashed.contains(&i) {
                    None
                } else {
                    Some(ct.clone())
                }
            })
            .collect();
    }
}

fn part1(carts: &[Cart], grid: &Grid) -> Coord {
    let mut carts = carts.to_vec();

    for i in 0_usize.. {
        if let Some(coord) = attempt_collide(&mut carts, grid) {
            return coord;
        }
    }

    (0, 0)
}

fn attempt_collide(carts: &mut Vec<Cart>, grid: &Grid) -> Option<Coord> {
    carts.sort_by(|Cart((a_x, a_y), _, _), Cart((b_x, b_y), _, _)| {
        let a = a_y * WIDTH + a_x;
        let b = b_y * WIDTH + b_x;
        a.cmp(&b)
    });

    let mut locations: Vec<Coord> = carts
        .iter()
        .map(|Cart(coord, _, _)| coord.clone())
        .collect();

    for (i, cart) in carts.iter_mut().enumerate() {
        cart.next_frame(grid);
        if locations.iter().any(|c| c == &cart.0) {
            return Some(cart.0);
        }
        locations[i] = cart.0;
    }

    None
}

#[derive(Debug, PartialEq)]
enum Track {
    Nothing,
    Junction,
    TrackLR,
    TrackUD,
    CurveR,
    CurveL,
}

type Coord = (usize, usize);
type Velocity = (isize, isize);
type Grid = Array2<Track>;

#[derive(Debug, Clone)]
struct Cart(Coord, Velocity, usize);

impl Cart {
    fn turn_left(&mut self) {
        self.1 = match self.1 {
            UP => LEFT,
            DOWN => RIGHT,
            LEFT => DOWN,
            RIGHT => UP,
            _ => unreachable!(),
        };
    }

    fn turn_right(&mut self) {
        self.1 = match self.1 {
            UP => RIGHT,
            DOWN => LEFT,
            LEFT => UP,
            RIGHT => DOWN,
            _ => unreachable!(),
        };
    }

    fn advance(&mut self) {
        let (x, y) = self.0;
        let (vx, vy) = self.1;
        let (x, y) = (x as isize + vx, y as isize + vy);
        self.0 = (x as usize, y as usize);
    }

    fn next_frame(&mut self, grid: &Array2<Track>) {
        let coord = self.0;
        let t = grid.get(grid_coord(&coord)).unwrap();
        match t {
            Track::Nothing => unreachable!(),
            Track::TrackLR => self.advance(),
            Track::TrackUD => self.advance(),
            Track::Junction => {
                let decision = self.2 % 3;
                match decision {
                    0 => {
                        self.turn_left();
                        self.advance();
                    }
                    1 => self.advance(),
                    2 => {
                        self.turn_right();
                        self.advance();
                    }
                    _ => unreachable!(),
                };
                self.2 += 1;
            }
            Track::CurveR => {
                let direction = self.1;
                self.1 = match direction {
                    UP => RIGHT,
                    RIGHT => UP,
                    DOWN => LEFT,
                    LEFT => DOWN,
                    _ => unreachable!(),
                };
                self.advance();
            }
            Track::CurveL => {
                let direction = self.1;
                self.1 = match direction {
                    UP => LEFT,
                    RIGHT => DOWN,
                    LEFT => UP,
                    DOWN => RIGHT,
                    _ => unreachable!(),
                };
                self.advance();
            }
        }
    }
}

// it's row, col instead of what I expect in here
fn grid_coord(coord: &Coord) -> Coord {
    (coord.1, coord.0)
}

fn parse_input(buf: &str) -> (Grid, Vec<Cart>) {
    let carts: Vec<Cart> = buf
        .lines()
        .enumerate()
        .map(|(y, ln)| {
            ln.chars().enumerate().filter_map(move |(x, c)| match c {
                '>' => Some(Cart((x, y), RIGHT, 0)),
                '<' => Some(Cart((x, y), LEFT, 0)),
                '^' => Some(Cart((x, y), UP, 0)),
                'v' => Some(Cart((x, y), DOWN, 0)),
                _ => None,
            })
        })
        .flatten()
        .collect();

    // println!("buf \n{}", buf);

    let grid: Vec<Track> = buf
        .lines()
        .map(|ln| ln.chars())
        .flatten()
        .map(|c| match c {
            '|' => Track::TrackUD,
            '-' => Track::TrackLR,
            '+' => Track::Junction,
            '\\' => Track::CurveL,
            '/' => Track::CurveR,
            '>' => Track::TrackLR,
            '<' => Track::TrackLR,
            '^' => Track::TrackUD,
            'v' => Track::TrackUD,
            _ => Track::Nothing,
        })
        .collect();
    assert_eq!(WIDTH * WIDTH, grid.len());
    let grid = Array2::from_shape_vec((WIDTH, WIDTH), grid).unwrap();

    (grid, carts)
}

#[cfg(test)]
mod test {
    use super::*;

    //     #[test]
    //     fn test_part2() {
    //         let buf = r#"/>-<\
    // |   |
    // | /<+-\
    // | | | v
    // \>+</ |
    //   |   ^
    //   \<->/"#;
    //         let (grid, carts) = parse_input(&buf);
    //         let p2 = part2(&carts, &grid);
    //         assert_eq!((6, 4), p2);
    //     }

    #[test]
    fn test_part1_real() {
        let buf = include_str!("../../input-day13");
        let (grid, carts) = parse_input(&buf);
        assert_eq!((69, 46), part1(&carts, &grid));
    }

    #[test]
    fn test_part2_real() {
        let buf = include_str!("../../input-day13");
        let (grid, carts) = parse_input(&buf);
        assert_eq!((118, 108), part2(&carts, &grid));
    }
}
