#[macro_use]
extern crate nom;
extern crate chrono;
extern crate tap;

use chrono::prelude::*;
use chrono::Duration;
use nom::digit;
use nom::types::CompleteStr;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use tap::TapOps;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("input-day4")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let logs: Vec<Log> = buf
        .lines()
        .map(|l| parse_log(l.into()).unwrap().1)
        .collect();

    println!("day4.1 {}", part1(&logs));
    println!("day4.2 {}", part2(&logs));

    Ok(())
}

// A = guard with most minutes asleep
// B = which minute that guard spent asleep most
// A*B
fn part1(logs: &[Log]) -> usize {
    let mut logs = logs.to_vec();
    logs.sort_by(|a, b| a.ts.cmp(&b.ts));

    let mut sleep_accum: HashMap<String, Duration> = HashMap::new();
    let mut sleep_ranges: HashMap<String, Vec<std::ops::Range<u8>>> = HashMap::new();
    let mut guard = "BOB".to_string();
    let mut asleep = Utc::now();
    let mut sleep_minute = 0;
    for log in logs {
        match log.state {
            GuardState::BeginShift(g) => guard = g.clone(),
            GuardState::Asleep => {
                asleep = log.ts;
                sleep_minute = log.minute;
            }
            GuardState::Wake => {
                sleep_accum
                    .entry(guard.clone())
                    .and_modify(|e| *e = *e + log.ts.signed_duration_since(asleep))
                    .or_insert(log.ts.signed_duration_since(asleep));
                sleep_ranges
                    .entry(guard.clone())
                    .and_modify(|e| e.push(sleep_minute..log.minute))
                    .or_insert(vec![sleep_minute..log.minute]);
            }
        }
    }

    let sleepiest_guard =
        sleep_accum
            .iter()
            .fold(("", Duration::seconds(0)), |acc, (guard, naptime)| {
                if naptime > &acc.1 {
                    (guard, *naptime)
                } else {
                    acc
                }
            });
    let sleep_slots: HashMap<u8, usize> =
        sleep_ranges[sleepiest_guard.0]
            .iter()
            .fold(HashMap::new(), |mut acc, r| {
                for i in r.clone() {
                    acc.entry(i).and_modify(|e| *e += 1).or_insert(1);
                }
                acc
            });
    let most_slept: (u8, usize) = sleep_slots.iter().fold((0, 0), |mut acc, tup| {
        if tup.1 > &acc.1 {
            acc = (tup.0.clone(), tup.1.clone());
        }
        acc
    });

    let guard_num: usize = sleepiest_guard.0.parse().unwrap();
    guard_num * most_slept.0 as usize
}

fn part2(logs: &[Log]) -> usize {
    let mut logs = logs.to_vec();
    logs.sort_by(|a, b| a.ts.cmp(&b.ts));

    let mut sleep_ranges: HashMap<String, Vec<std::ops::Range<u8>>> = HashMap::new();
    let mut guard = "BOB".to_string();
    let mut sleep_minute = 0;
    for log in logs {
        match log.state {
            GuardState::BeginShift(g) => guard = g.clone(),
            GuardState::Asleep => {
                sleep_minute = log.minute;
            }
            GuardState::Wake => {
                sleep_ranges
                    .entry(guard.clone())
                    .and_modify(|e| e.push(sleep_minute..log.minute))
                    .or_insert(vec![sleep_minute..log.minute]);
            }
        }
    }

    let consistent_sleeper: Vec<(String, (u8, usize))> =
        sleep_ranges
            .iter()
            .fold(Vec::new(), |acc, (guard, sleepytimes)| {
                let sleep_slots: HashMap<u8, usize> =
                    sleepytimes.iter().fold(HashMap::new(), |mut acc, r| {
                        for i in r.clone() {
                            acc.entry(i).and_modify(|e| *e += 1).or_insert(1);
                        }
                        acc
                    });
                let preferred_sleepytime: (u8, usize) = sleep_slots.iter().fold(
                    (0u8, 0usize),
                    |(a_minute, a_count), (&minute, &count)| {
                        if count > a_count {
                            (minute, count)
                        } else {
                            (a_minute, a_count)
                        }
                    },
                );
                acc.tap(|a| a.push((guard.clone(), preferred_sleepytime)))
            });

    let winner: (String, (u8, usize)) = consistent_sleeper.iter().fold(
        ("".into(), (0, 0)),
        |acc, (guard, (ref minute, ref freq))| {
            let (a_guard, (a_minute, a_freq)) = acc;
            if freq > &a_freq {
                (guard.to_string(), (*minute, *freq))
            } else {
                (a_guard, (a_minute, a_freq))
            }
        },
    );
    let (guard, (minute, _)) = winner;
    let guard: usize = guard.parse().unwrap();
    guard * minute as usize
}

#[derive(Debug, PartialEq, Clone)]
enum GuardState {
    BeginShift(String),
    Asleep,
    Wake,
}

#[derive(Debug, PartialEq, Clone)]
struct Log {
    ts: DateTime<Utc>,
    minute: u8,
    state: GuardState,
}

named!(
    parse_sleep<CompleteStr, GuardState>,
    do_parse!(tag!("falls asleep") >> (GuardState::Asleep))
);

named!(
    parse_wake<CompleteStr, GuardState>,
    do_parse!(tag!("wakes up") >> (GuardState::Wake))
);

named!(
    parse_shift<CompleteStr, GuardState>,
    do_parse!(
        tag!("Guard #") >> num: digit >> tag!(" begins shift") >> (GuardState::BeginShift(num.as_ref().into()))
    )
);

named!(
    parse_log<CompleteStr, Log>,
    do_parse!(
        tag!("[")
            >> dt: take!(16)
            >> tag!("] ")
            >> state: alt!(parse_sleep | parse_wake | parse_shift)
            >> (Log {
                ts: Utc.datetime_from_str(&dt, "%Y-%m-%d %H:%M").unwrap(),
                minute: dt.as_ref().split(':').skip(1).next().map(str::parse::<u8>).unwrap().unwrap(),
                state: state,
            })
    )
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_log() {
        assert_eq!(
            parse_log("[1518-08-21 00:39] wakes up".into()),
            Ok((
                "".into(),
                Log {
                    ts: Utc.ymd(1518, 8, 21).and_hms(0, 39, 0),
                    minute: 39,
                    state: GuardState::Wake,
                }
            ))
        );
    }

    #[test]
    fn test_part_1() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let logs: Vec<Log> = input
            .lines()
            .map(|l| parse_log(l.into()).unwrap().1)
            .collect();
        assert_eq!(10 * 24, part1(&logs));
    }

    #[test]
    fn test_real_part1() {
        let buf = include_str!("../../input-day4");
        let logs: Vec<Log> = buf
            .lines()
            .map(|l| parse_log(l.into()).unwrap().1)
            .collect();
        assert_eq!(77084, part1(&logs));
    }

    #[test]
    fn test_part_2() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let logs: Vec<Log> = input
            .lines()
            .map(|l| parse_log(l.into()).unwrap().1)
            .collect();
        assert_eq!(99 * 45, part2(&logs));
    }

    #[test]
    fn test_real_part2() {
        let buf = include_str!("../../input-day4");
        let logs: Vec<Log> = buf
            .lines()
            .map(|l| parse_log(l.into()).unwrap().1)
            .collect();
        assert_eq!(23047, part2(&logs));
    }
}
