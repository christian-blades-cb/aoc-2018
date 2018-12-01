use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input-day1")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let mods = parse_input(&buf);

    let end_freq = freq(&mods);
    println!("day1.1 {}", end_freq);

    let first_twice = twice(&mods);
    println!("day1.2 {}", first_twice);

    Ok(())
}

fn parse_input(buf: &[u8]) -> Vec<isize> {
    buf.split(|x| *x == b'\n')
        .filter_map(|l| {
            if l.len() < 2 {
                return None;
            }
            let muh = std::str::from_utf8(&l[1..]).unwrap();
            let num: isize = muh.parse().unwrap();
            let res = match l[0] {
                b'+' => num,
                b'-' => 0 - num,
                _ => 0,
            };
            Some(res)
        })
        .collect()
}

fn freq(mods: &[isize]) -> isize {
    mods.iter().fold(0isize, |acc, x| acc + x)
}

fn twice(mods: &[isize]) -> isize {
    let mut uniqs: HashMap<isize, usize> = HashMap::new();
    let mut start = 0isize;
    uniqs.insert(start, 1);
    loop {
        let freq = mods.iter().scan(start, |acc, x| {
            *acc += x;
            Some(*acc)
        });
        start = mods.iter().fold(start, |acc, x| acc + x);

        for x in freq {
            let ent = uniqs.entry(x).and_modify(|e| *e = *e + 1).or_insert(1);
            if *ent == 2 {
                return x;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_twice() {
        let input = vec![1, -1];
        assert_eq!(twice(&input), 0);
        let input = vec![3, 3, 4, -2, -4];
        assert_eq!(twice(&input), 10);
        let input = vec![-6, 3, 8, 5, -6];
        assert_eq!(twice(&input), 5);
        let input = vec![7, 7, -2, -7, -4];
        assert_eq!(twice(&input), 14);
    }

    #[test]
    fn test_day1() {
        let buf = include_bytes!("../../input-day1");
        let mods = parse_input(buf);
        assert_eq!(freq(&mods), 474);
    }

    #[test]
    fn test_day2() {
        let buf = include_bytes!("../../input-day1");
        let mods = parse_input(buf);
        assert_eq!(twice(&mods), 137041);
    }
}
