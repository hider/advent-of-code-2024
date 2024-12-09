use adv_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::cmp::Ordering;
use std::fs;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug)]
struct Rule {
    a: u16,
    b: u16,
}

impl FromIterator<u16> for Rule {
    fn from_iter<T: IntoIterator<Item = u16>>(iter: T) -> Self {
        let numbers: Vec<_> = iter.into_iter().take(2).collect();
        assert_eq!(numbers.len(), 2);
        Rule {
            a: numbers[0],
            b: numbers[1],
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);
    let data: String = fs::read_to_string(INPUT_FILE)?;

    let data: Vec<_> = data.splitn(2, "\n\n").take(2).collect();
    assert_eq!(data.len(), 2);

    let rules: Vec<Rule> = data[0]
        .lines()
        .map(|line| {
            line.splitn(2, "|")
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let updates: Vec<Vec<u16>> = data[1]
        .lines()
        .map(|line| {
            let update: Vec<_> = line.split(",").map(|num| num.parse().unwrap()).collect();
            assert_eq!(update.len() % 2, 1);
            update
        })
        .collect();

    let ordering_rule = |a, b| -> Ordering {
        match rules
            .iter()
            .find(|rule| rule.a == a && rule.b == b || rule.a == b && rule.b == a)
        {
            Some(rule) => {
                if rule.a == a && rule.b == b {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            None => Ordering::Equal,
        }
    };

    let sorting_rule = |a, b| -> bool {
        match ordering_rule(a, b) {
            Ordering::Less => false,
            Ordering::Equal => true,
            Ordering::Greater => true,
        }
    };

    let ordered_unordered: (Vec<_>, Vec<_>) = updates
        .iter()
        .partition(|update| update.iter().is_sorted_by(|&&a, &&b| sorting_rule(a, b)));

    let sum: usize = ordered_unordered
        .0
        .iter()
        .map(|update| update[update.len() / 2] as usize)
        .sum();
    println!("Part 1 sum: {sum}");
    assert_eq!(6260, sum);

    let sum: usize = ordered_unordered
        .1
        .iter()
        .map(|&update| {
            let mut update = update.clone();
            update.sort_unstable_by(|&a, &b| ordering_rule(a, b));
            update
        })
        .map(|update| update[update.len() / 2] as usize)
        .sum();

    println!("Part 2 sum: {sum}");
    assert_eq!(5346, sum);

    Ok(())
}
