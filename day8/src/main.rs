use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;

    let mut file = File::open("input-day8")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let input = parse_input(&buf);
    println!("day8.1 {}", part1(&input));
    println!("day8.2 {}", part2(&input));

    Ok(())
}

fn part1(input: &[u8]) -> usize {
    let (_, metadata) = metadater(input);
    metadata
}

// offset, metadata_sum
fn metadater(input: &[u8]) -> (usize, usize) {
    let children_n = input[0];
    let metadata_n = input[1] as usize;
    let mut offset = 2;
    let mut metadata = 0;
    for _n in 0..children_n {
        let (child_off, child_meta) = metadater(&input[offset..]);
        metadata += child_meta;
        offset += child_off;
    }
    let my_metadata: usize = input[offset..]
        .iter()
        .cloned()
        .take(metadata_n)
        .map(|x| x as usize)
        .sum();
    metadata += my_metadata;
    offset += metadata_n;
    (offset, metadata)
}

fn parse_input(buf: &str) -> Vec<u8> {
    buf.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn part2(input: &[u8]) -> usize {
    let (_, metadata) = metachild(input);
    metadata
}

// offset, metadata_sum/val
fn metachild(input: &[u8]) -> (usize, usize) {
    let children_n = input[0];
    let metadata_n = input[1] as usize;
    let mut offset = 2;
    let mut child_metadata: Vec<usize> = Vec::new();
    for _n in 0..children_n {
        let (child_off, child_meta) = metachild(&input[offset..]);
        child_metadata.push(child_meta);
        offset += child_off;
    }
    let my_metadata: usize = match children_n {
        0 => input[offset..]
            .iter()
            .cloned()
            .take(metadata_n)
            .map(|x| x as usize)
            .sum(),
        _ => input[offset..]
            .iter()
            .cloned()
            .take(metadata_n)
            .map(|i| {
                child_metadata
                    .get(i as usize - 1)
                    .cloned()
                    .unwrap_or(0usize)
            })
            .sum(),
    };
    offset += metadata_n;
    (offset, my_metadata)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let buf = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let input = parse_input(&buf);

        assert_eq!(138, part1(&input));
    }

    #[test]
    fn test_part2() {
        let buf = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let input = parse_input(&buf);

        assert_eq!(66, part2(&input));
    }

    fn real_input() -> Vec<u8> {
        let buf = include_str!("../../input-day8");
        parse_input(&buf)
    }

    #[test]
    fn test_part1_real() {
        let input = real_input();
        assert_eq!(40977, part1(&input));
    }

    #[test]
    fn test_part2_real() {
        let input = real_input();
        assert_eq!(27490, part2(&input));
    }
}
