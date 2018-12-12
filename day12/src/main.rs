#[macro_use]
extern crate nom;

use nom::types::CompleteStr;
use std::collections::*;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;
    let mut file = File::open("input-day12")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let (initial_state, patterns) = parse_input(&buf);

    println!("day12.1 {}", part1(&initial_state, &patterns));
    println!("day12.2 {}", part2(&initial_state, &patterns));

    Ok(())
}

type Patterns = HashMap<[bool; 5], bool>;

fn part1(initial_state: &[bool], patterns: &Patterns) -> isize {
    const padding: usize = 50;
    let mut state: Vec<bool> = vec![false; padding];
    state.append(&mut initial_state.to_vec());
    state.append(&mut vec![false; padding]);

    let final_state = (0..20).fold(state, |acc, _| generation(&acc, patterns));

    final_state
        .iter()
        .enumerate()
        .map(|(i, x)| if *x { i as isize - padding as isize } else { 0 })
        .sum()
}

fn part2(initial_state: &[bool], patterns: &Patterns) -> isize {
    const padding: usize = 1000;
    const n_generations: usize = 50_000_000_000;
    let mut state: Vec<bool> = vec![false; padding];
    state.append(&mut initial_state.to_vec());
    state.append(&mut vec![false; padding]);

    let generations = (0..n_generations).scan(state, |acc, _| {
        let next_gen = generation(&acc, patterns);
        *acc = next_gen.clone();

        Some(next_gen)
    });

    let mut seen = HashMap::new();

    let mut plant_sum = 0;

    for (n, gen) in generations.enumerate() {
        let gen_sum: isize = gen
            .iter()
            .enumerate()
            .map(|(i, x)| if *x { i as isize - padding as isize } else { 0 })
            .sum();
        // println!("[{}] -> {}", n, gen_sum);
        plant_sum = gen_sum;
        let lbounds = gen.iter().position(|x| *x == true).unwrap();
        let rbounds = gen.iter().rposition(|x| *x == true).unwrap();
        let pattern = gen[lbounds..=rbounds].to_vec();
        let current_pos = lbounds as isize - padding as isize;
        if let Some((first_pos, _first_gen)) = seen.insert(pattern.clone(), (current_pos, n)) {
            // println!(
            //     "convergence! position: {}, generation: {}",
            //     first_pos, _first_gen
            // );

            let pos_diff = current_pos - first_pos;
            let remaining_generations = n_generations - n - 1;
            let final_pos = (pos_diff * remaining_generations as isize) + current_pos;

            return pattern
                .iter()
                .enumerate()
                .map(|(i, x)| if *x { i as isize + final_pos } else { 0 })
                .sum();
        }
    }

    plant_sum
}

fn generation(initial_state: &[bool], patterns: &Patterns) -> Vec<bool> {
    let mut generation: Vec<bool> = Vec::with_capacity(initial_state.len());
    generation.push(initial_state[0]);
    generation.push(initial_state[1]);
    for i in 2..initial_state.len() - 2 {
        match patterns.get(&[
            initial_state[i - 2],
            initial_state[i - 1],
            initial_state[i],
            initial_state[i + 1],
            initial_state[i + 2],
        ]) {
            Some(x) => generation.push(*x),
            None => generation.push(initial_state[i]),
        }
    }
    generation.push(initial_state[initial_state.len() - 2]);
    generation.push(initial_state[initial_state.len() - 1]);

    generation
}

fn parse_input(buf: &str) -> (Vec<bool>, Patterns) {
    let mut lines = buf.lines();
    let initial_state = parse_initial_state(lines.next().unwrap().into()).unwrap().1;
    let patterns: HashMap<[bool; 5], bool> = lines
        .skip(1)
        .map(|ln| {
            let (pat, result) = parse_pattern(ln.into()).unwrap().1;
            let pat_arr: [bool; 5] = [pat[0], pat[1], pat[2], pat[3], pat[4]];
            (pat_arr, result)
        })
        .collect();

    (initial_state, patterns)
}

named!(parse_state<CompleteStr, bool>,
       do_parse!(
           st: alt!(tag!(".") | tag!("#")) >>
               (match st.as_ref() {
                   "." => false,
                   "#" => true,
                   _ => unreachable!(),
               })));

named!(parse_initial_state<CompleteStr, Vec<bool>>,
       do_parse!(
           tag!("initial state: ") >>
               states: many1!(parse_state) >>
               (states)));

named!(parse_pattern<CompleteStr, (Vec<bool>, bool)>,
       do_parse!(
           states: many1!(parse_state) >>
               tag!(" => ") >>
               end_state: parse_state >>
               ((states, end_state))));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_real() {
        let buf = include_str!("../../input-day12");
        let (initial_state, patterns) = parse_input(&buf);
        assert_eq!(4386, part1(&initial_state, &patterns));
    }

    #[test]
    fn test_part2_real() {
        let buf = include_str!("../../input-day12");
        let (initial_state, patterns) = parse_input(&buf);
        assert_eq!(5450000001166, part2(&initial_state, &patterns));
    }
}
