use std::io::prelude::*;
use tap::TapOps;

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;

    let mut file = File::open("input-day14")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    // let input = 768071;
    let input: usize = buf.parse().unwrap();

    println!("day14.1 {}", part1(input));
    println!("day14.2 {}", part2(input));

    Ok(())
}

struct Elf(usize);

fn part1(input: usize) -> String {
    let mut recipes = vec![3_usize, 7];
    let mut elf1 = Elf(0);
    let mut elf2 = Elf(1);

    while recipes.len() < input + 10 {
        let a = recipes[elf1.0];
        let b = recipes[elf2.0];
        let mut new_recipes: Vec<usize> = {
            let x = a + b;
            let x = format!("{}", x);
            x.bytes().map(|b| (b - 48) as usize).collect()
        };

        recipes.append(&mut new_recipes);

        elf1.0 = (elf1.0 + 1 + a) % recipes.len();
        elf2.0 = (elf2.0 + 1 + b) % recipes.len();
    }

    recipes
        .iter()
        .skip(input)
        .take(10)
        .fold(String::new(), |acc, x| {
            acc.tap(|a| a.push(((x + 48) as u8).into()))
        })
}

fn part2(input: usize) -> usize {
    const BATCH_SIZE: usize = 10_000;
    let mut recipes = vec![3_usize, 7];
    let mut elf1 = Elf(0);
    let mut elf2 = Elf(1);

    let pattern: String = format!("{}", input);
    let pattern_len = pattern.len();
    let mut haystack = String::with_capacity(BATCH_SIZE * 2); // re-use to reduce allocs
    let mut digit_accum = Vec::new(); // also for re-use to avoid allocs

    loop {
        let prev_len = recipes.len();

        recipes.reserve(BATCH_SIZE * 2); // less frequent allocs
        for _ in 0..BATCH_SIZE {
            let a = recipes[elf1.0];
            let b = recipes[elf2.0];
            {
                let x = a + b;
                digit_accum.clear();
                to_digits(x, &mut digit_accum);
                recipes.append(&mut digit_accum);
            }

            elf1.0 = (elf1.0 + 1 + a) % recipes.len();
            elf2.0 = (elf2.0 + 1 + b) % recipes.len();
        }

        let offset = prev_len.saturating_sub(pattern_len); // all this offset nonsense to reduce size of the haystack

        haystack.clear();
        for d in recipes[offset..].iter() {
            let c = (d + 48) as u8;
            haystack.push(c.into());
        }

        if let Some(idx) = haystack.find(pattern.as_str()) {
            return idx + offset;
        }
    }
}

fn to_digits(n: usize, acc: &mut Vec<usize>) {
    if n >= 10 {
        to_digits(n / 10, acc);
    }
    acc.push(n % 10);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("0124515891", part1(5));
        assert_eq!("9251071085", part1(18));
        assert_eq!("5941429882", part1(2018));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2(51589));
        assert_eq!(18, part2(92510));
        assert_eq!(2018, part2(59414));
    }

    #[test]
    fn test_part1_real() {
        let input = 768071;
        assert_eq!("6548103910", part1(input));
    }

    #[test]
    fn test_part2_real() {
        let input = 768071;
        assert_eq!(20198090, part2(input));
    }
}
