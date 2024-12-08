use adv_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::fs;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");


fn main() -> Result<()> {
    start_day(DAY);

    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();
    let data: String = fs::read_to_string(INPUT_FILE)?;

    data.lines().for_each(|line| {
        let elems = line.split_whitespace()
            .map(|elem| elem.parse::<u32>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap();
        first.push(*elems.get(0).unwrap());
        second.push(*elems.get(1).unwrap());
    });
    first.sort_unstable();
    second.sort_unstable();

    let sum: u32 = first.iter()
        .zip(&second)
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    println!("Sum: {sum}");

    let similarity: usize = first.iter()
        .map(|num| second.iter()
            .filter(|n| *n == num)
            .count()
            .checked_mul(*num as usize)
            .unwrap()
        )
        .sum();

    println!("Similarity: {similarity}");

    assert_eq!(2904518, sum);
    assert_eq!(18650129, similarity);

    Ok(())
}
