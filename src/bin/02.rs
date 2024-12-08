use adv_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::cmp::Ordering;
use std::fs;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn main() -> Result<()> {
    start_day(DAY);

    let data: String = fs::read_to_string(INPUT_FILE)?;

    let mut counter: usize = 0;
    data.lines().for_each(|line| {
        let levels = levels(line);

        let mut safe = false;
        let mut prev: u16 = 0;
        let mut ordering = Ordering::Equal;
        for level in levels {
            if prev == 0 {
                prev = level;
                continue;
            }
            if (1..=3).contains(&prev.abs_diff(level)) {
                let cmp = prev.cmp(&level);
                if ordering.is_eq() {
                    ordering = cmp;
                }
                if ordering == cmp {
                    safe = true;
                    prev = level;
                    continue;
                }
            }
            safe = false;
            break;
        }

        if safe {
            counter += 1;
        }
    });

    println!("Safe: {counter}");
    assert_eq!(314, counter);

    counter = 0;
    data.lines().for_each(|line| {
        let levels = levels(line);

        let mut safe = false;
        let mut bad_level = false;
        let mut prev: u16 = 0;
        let mut ordering = Ordering::Equal;
        for level in levels {
            if prev == 0 {
                prev = level;
                continue;
            }
            if (1..=3).contains(&prev.abs_diff(level)) {
                let cmp = prev.cmp(&level);
                if ordering.is_eq() {
                    ordering = cmp;
                }
                if ordering == cmp {
                    safe = true;
                    prev = level;
                    continue;
                }
            }
            if bad_level {
                safe = false;
                break;
            }
            bad_level = true;
            safe = true;
            // part 2 doesn't work with the example with the line below
            prev = level;
        }

        if safe {
            counter += 1;
        }
    });

    println!("Safe dampened: {counter}");
    assert_eq!(373, counter);

    Ok(())
}

fn levels(line: &str) -> Vec<u16> {
    line.split_whitespace()
        .map(|elem| elem.parse::<u16>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}
