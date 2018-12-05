use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input-day5")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    println!("day5.1 {}", part1(&buf));
    println!("day5.2 {}", part2(&buf));

    Ok(())
}

fn part1(buf: &[u8]) -> usize {
    let mut buf = buf.to_vec();
    loop {
        let mut new_buf: Vec<u8> = Vec::with_capacity(buf.len());
        for c in buf.iter() {
            let current = new_buf.pop();
            if current.is_none() {
                new_buf.push(*c);
                continue;
            }
            let current = current.unwrap();
            if current == c - 32 || *c == current - 32 {
                continue; // reacted
            }
            new_buf.push(current);
            new_buf.push(*c);
        }
        if new_buf.len() == buf.len() {
            break;
        }
        buf = new_buf
    }
    buf.len()
}

fn part2(buf: &[u8]) -> usize {
    let filtered_lens = (65..=90_u8).into_iter().map(|n| {
        let new_buf: Vec<u8> = buf
            .iter()
            .filter(|&&c| c != n && c != n + 32)
            .map(|x| *x)
            .collect();
        part1(&new_buf)
    });
    let shortest_chain = filtered_lens.fold(
        std::usize::MAX,
        |acc, ref len| if len < &acc { *len } else { acc },
    );
    shortest_chain
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let buf = "dabAcCaCBAcCcaDA".as_bytes();
        assert_eq!(10, part1(&buf));
    }

    #[test]
    fn test_part2() {
        let buf = "dabAcCaCBAcCcaDA".as_bytes();
        assert_eq!(4, part2(&buf));
    }

    #[test]
    fn test_part1_real() {
        let buf = include_bytes!("../../input-day5");
        assert_eq!(11590, part1(buf));
    }

    #[test]
    fn test_part2_real() {
        let buf = include_bytes!("../../input-day5");
        assert_eq!(4504, part2(buf));
    }
}
