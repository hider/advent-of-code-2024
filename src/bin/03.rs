use adv_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use regex::Regex;
use std::fs;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const DO: &str = "do()";
const DO_LEN: usize = DO.len();

const DONT: &str = "don't()";
const DONT_LEN: usize = DONT.len();

fn main() -> Result<()> {
    start_day(DAY);

    let data: String = fs::read_to_string(INPUT_FILE)?;

    let sum = multiple_and_sum(&data);

    println!("Sum: {sum}");
    assert_eq!(175700056, sum);

    let do_sum: usize = find_do_slices(&data)
        .into_iter()
        .map(multiple_and_sum)
        .sum();

    println!("Sum of do(): {do_sum}");

    assert_eq!(71668682, do_sum);

    Ok(())
}

fn multiple_and_sum(data: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(data)
        .map(|c| c.extract().1)
        .map(|[x, y]| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .map(|(x, y)| x * y)
        .sum()
}

fn find_do_slices(data: &str) -> Vec<&str> {
    let do_slice = match data.find(DONT) {
        Some(index) => (index, &data[..index]),
        None => (0, &data[..]),
    };

    let mut result = vec![do_slice.1];

    if do_slice.0 != 0 {
        let from = do_slice.0 + DONT_LEN;
        let sub_string = &data[from..];
        sub_string
            .find(DO)
            .map(|index| &sub_string[index + DO_LEN..])
            .map(find_do_slices)
            .map(|v| result.extend(v));
    }

    result
}
