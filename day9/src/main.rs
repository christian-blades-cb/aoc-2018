#[macro_use]
extern crate nom;

use nom::digit;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;

    let mut file = File::open("input-day9")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let (n_players, last_marble) = parse_input(&buf).unwrap().1;

    println!("day9.1 {}", part2(n_players, last_marble));
    println!("day9.2 {}", part2(n_players, last_marble * 100));

    Ok(())
}

fn part2(n_players: usize, final_marble: usize) -> usize {
    use std::collections::VecDeque;

    let mut board = VecDeque::with_capacity(final_marble);
    board.push_back(0);
    let mut players: Vec<usize> = vec![0_usize; n_players];
    let mut current_player = (0..players.len()).into_iter().cycle().skip(1);
    let draw = (1..=final_marble).into_iter();

    for marble in draw {
        let player_i = current_player.next().unwrap();

        if marble % 23 == 0 {
            players[player_i] += marble;
            for _n in 0..7 {
                let back = board.pop_back().unwrap();
                board.push_front(back);
            }
            players[player_i] += board.pop_front().unwrap();
        } else {
            for _n in 0..2 {
                let front = board.pop_front().unwrap();
                board.push_back(front);
            }
            board.push_front(marble);
        }
    }

    *(players.iter().max().unwrap())
}

named!(parse_input<&str, (usize, usize)>,
       do_parse!(
           n_players: digit >>
               tag!(" players; last marble is worth ") >>
               last_marble: digit >>
               ((n_players.parse::<usize>().unwrap(),
                 last_marble.parse::<usize>().unwrap()))));

#[derive(Debug, Clone)]
struct Player(usize);

// so very inefficient, I assume it's spending a bunch of time
// shifting the elements in the `remove` function
#[allow(dead_code)]
fn part1(n_players: usize, final_marble: usize) -> usize {
    let mut board: Vec<usize> = Vec::with_capacity(final_marble);
    board.push(0);
    board.push(1);
    let mut players: Vec<Player> = vec![Player(0); n_players];
    let draw = (2..=final_marble).into_iter();
    let mut current_index: usize = 1;
    let mut current_player = (0..n_players).into_iter().cycle().skip(1);

    for marble in draw {
        let player_i = current_player.next().unwrap();

        if marble % 23 == 0 {
            players[player_i].0 += marble;
            let mut next_board_index: isize = current_index as isize - 7;
            if next_board_index < 0 {
                next_board_index =
                    board.len() as isize - (next_board_index.abs() % board.len() as isize);
            }
            players[player_i].0 += board.remove(next_board_index as usize);
            current_index = next_board_index as usize;
        } else {
            // non-23
            let next_board_index = (current_index + 2) % board.len();
            board.insert(next_board_index, marble);
            current_index = next_board_index;
        }
    }

    players.iter().map(|x| x.0).max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(32, part1(9, 25));
        assert_eq!(8317, part1(10, 1618));
        assert_eq!(146373, part1(13, 7999));
        assert_eq!(2764, part1(17, 1104));
        assert_eq!(54718, part1(21, 6111));
        assert_eq!(37305, part1(30, 5807));
    }

    #[test]
    fn test_part2() {
        assert_eq!(32, part2(9, 25));
        assert_eq!(8317, part2(10, 1618));
        assert_eq!(146373, part2(13, 7999));
        assert_eq!(2764, part2(17, 1104));
        assert_eq!(54718, part2(21, 6111));
        assert_eq!(37305, part2(30, 5807));
    }

    #[test]
    fn test_part1_real() {
        assert_eq!(367802, part2(493, 71863));
    }

    #[test]
    fn test_part2_real() {
        assert_eq!(2996043280, part2(493, 71863 * 100));
    }
}
