#[macro_use]
extern crate nom;
extern crate chrono;
extern crate itertools;
extern crate regex;
extern crate tap;
extern crate topological_sort;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use tap::TapOps;
use topological_sort::TopologicalSort;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input-day7")?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    let input: Vec<(u8, u8)> = buf
        .split(|x| *x == b'\n')
        .map(|ln| parse_step(ln).unwrap().1)
        .collect();

    println!("day7.1 {}", part1(&input));
    println!("day7.2 {}", part2(&input, 5, 60));
    Ok(())
}

fn part1(input: &[(u8, u8)]) -> String {
    use std::collections::HashSet;

    let mut popped_prec: HashSet<&u8> = HashSet::new();
    let mut order: Vec<&u8> = Vec::new();
    loop {
        let mut topology: TopologicalSort<&u8> = TopologicalSort::new();
        for (prec, succ) in input.iter().filter(|(p, _)| !popped_prec.contains(p)) {
            topology.add_dependency(prec, succ);
        }

        let mut to_order = topology.pop_all();
        if to_order.len() == 0 {
            break;
        }

        to_order.sort();
        let prec = to_order[0];
        order.push(prec);
        popped_prec.insert(prec);
    }

    // add everything that no longer has a predecessor
    let all_the_things: HashSet<&u8> = input.iter().fold(HashSet::new(), |acc, (a, b)| {
        acc.tap(|accum| {
            accum.insert(a);
            accum.insert(b);
        })
    });
    for succ in all_the_things.difference(&popped_prec) {
        order.push(succ);
    }

    let mut outstring = String::new();
    for x in order {
        outstring.push((*x).into());
    }
    outstring
}

#[inline]
fn letter_to_time(letter: &Option<u8>, time_per_step: usize) -> usize {
    // A == 1 seconds
    match letter {
        Some(l) => *l as usize - 64 + time_per_step,
        None => 0,
    }
}

#[derive(Clone, Debug)]
struct Worker {
    job: Option<u8>,
    done_t: usize,
}

fn part2(input: &[(u8, u8)], num_workers: usize, time_per_step: usize) -> usize {
    let mut popped_prec = HashSet::new();
    let mut working = HashSet::new();
    let mut workers = vec![
        Worker {
            job: None,
            done_t: 0
        };
        num_workers
    ];
    let mut output = String::new();

    let mut t = 0_usize;
    loop {
        // finish jobs
        for w in workers.iter().filter(|w| w.done_t <= t && w.job.is_some()) {
            let j = w.job.unwrap();
            output.push(j.into());
            popped_prec.insert(j);
        }

        // get available jobs
        let mut jobs: Vec<u8> = next_jobs(input, &popped_prec)
            .iter()
            .cloned()
            .filter(|x| !working.contains(x)) // not the ones that are/have been worked
            .collect();

        // assign new jobs
        for w in workers.iter_mut().filter(|w| w.done_t <= t) {
            let job = jobs.pop();
            if job.is_some() {
                working.insert(job.unwrap());
            }
            w.job = job;
            w.done_t = t + letter_to_time(&w.job, time_per_step);
        }

        // are we done?
        let all_done = workers.iter().all(|w| w.job.is_none());
        if all_done {
            break;
        }

        // tick
        t += 1
    }
    t
}

fn next_jobs(input: &[(u8, u8)], popped: &HashSet<u8>) -> Vec<u8> {
    let mut topology: TopologicalSort<&u8> = TopologicalSort::new();
    for (prec, succ) in input.iter().filter(|(p, _)| !popped.contains(p)) {
        topology.add_dependency(prec, succ);
    }

    let mut to_order = topology.pop_all();

    // whatever's left doesn't have a predecessor, so figure out what
    // we haven't processed and return that
    if to_order.len() == 0 {
        let all_the_things: HashSet<u8> =
            input.iter().cloned().fold(HashSet::new(), |acc, (a, b)| {
                acc.tap(|accum| {
                    accum.insert(a);
                    accum.insert(b);
                })
            });
        let mut succ: Vec<u8> = all_the_things
            .difference(popped)
            .into_iter()
            .cloned()
            .collect();
        succ.sort();
        return succ;
    }

    to_order.sort();
    return to_order.iter().map(|x| **x).collect();
}

named!(parse_step<&[u8], (u8, u8)>,
       do_parse!(
           tag!("Step ") >>
               prec: take!(1) >>
               tag!(" must be finished before step ") >>
               succ: take!(1) >>
               ((prec[0], succ[0]))));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let buf = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
            .as_bytes();
        let input: Vec<(u8, u8)> = buf
            .split(|x| *x == b'\n')
            .map(|ln| parse_step(ln).unwrap().1)
            .collect();
        assert_eq!("CABDFE", part1(&input));
    }

    #[test]
    fn test_part2() {
        let buf = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
            .as_bytes();
        let input: Vec<(u8, u8)> = buf
            .split(|x| *x == b'\n')
            .map(|ln| parse_step(ln).unwrap().1)
            .collect();
        assert_eq!(15, part2(&input, 2, 0));
    }

    fn real_input() -> Vec<(u8, u8)> {
        let buf = include_bytes!("../../input-day7");
        buf.split(|x| *x == b'\n')
            .map(|ln| parse_step(ln).unwrap().1)
            .collect()
    }

    #[test]
    fn test_part1_real() {
        let input = real_input();
        assert_eq!("EUGJKYFQSCLTWXNIZMAPVORDBH", part1(&input));
    }

    #[test]
    fn test_part2_real() {
        let input = real_input();
        assert_eq!(1014, part2(&input, 5, 60));
    }
}
