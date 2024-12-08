use adv_code_2024::*;
use anyhow::*;
use const_format::concatcp;
use std::fs;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn main() -> Result<()> {
    start_day(DAY);
    let data: String = fs::read_to_string(INPUT_FILE)?;

    // square matrices only
    let size = data.lines().next().unwrap().len();
    let mut matrix = vec![vec!['.'; size]; size];
    let mut sum: usize = 0;

    data.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            matrix[i][j] = c;
        });
    });

    assert_eq!(size, matrix.len());
    assert_eq!(size, matrix[matrix.len() - 1].len());
    assert_eq!(size, matrix.capacity());
    assert_eq!(size, matrix[matrix.len() - 1].capacity());

    for (i, line) in matrix.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if c == 'X' {
                if j+3 < size {
                    // left to right
                    if matrix[i][j+1] == 'M'
                        && matrix[i][j+2] == 'A'
                        && matrix[i][j+3] == 'S'
                    {
                        sum += 1;
                    }
                    // diagonal right up
                    if i.checked_sub(3).is_some()
                        && matrix[i-1][j+1] == 'M'
                        && matrix[i-2][j+2] == 'A'
                        && matrix[i-3][j+3] == 'S'
                    {
                        sum += 1;
                    }
                    // diagonal right down
                    if i + 3 < size
                        && matrix[i+1][j+1] == 'M'
                        && matrix[i+2][j+2] == 'A'
                        && matrix[i+3][j+3] == 'S'
                    {
                        sum += 1;
                    }
                }
                if j.checked_sub(3).is_some() {
                    // right to left
                    if matrix[i][j-1] == 'M'
                        && matrix[i][j-2] == 'A'
                        && matrix[i][j-3] == 'S'
                    {
                        sum += 1;
                    }
                    // diagonal left up
                    if i.checked_sub(3).is_some()
                        && matrix[i-1][j-1] == 'M'
                        && matrix[i-2][j-2] == 'A'
                        && matrix[i-3][j-3] == 'S'
                    {
                        sum += 1;
                    }
                    // diagonal left down
                    if i + 3 < size
                        && matrix[i+1][j-1] == 'M'
                        && matrix[i+2][j-2] == 'A'
                        && matrix[i+3][j-3] == 'S'
                    {
                        sum += 1;
                    }
                }
                // top to bottom
                if i+3 < size
                    && matrix[i+1][j] == 'M'
                    && matrix[i+2][j] == 'A'
                    && matrix[i+3][j] == 'S'
                {
                    sum += 1;
                }
                // bottom to top
                if i.checked_sub(3).is_some()
                    && matrix[i-1][j] == 'M'
                    && matrix[i-2][j] == 'A'
                    && matrix[i-3][j] == 'S'
                {
                    sum += 1;
                }
            }
        }
    }

    println!("XMAS sum: {}", sum);
    assert_eq!(2593, sum);

    sum = 0;

    for (i, line) in matrix.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if j + 2 < size
                && i + 2 < size
                && matrix[i+1][j + 1] == 'A'
            {
                // M.S
                if c == 'M'
                    && matrix[i][j + 2] == 'S'
                    && matrix[i + 2][j] == 'M'
                    && matrix[i + 2][j + 2] == 'S'
                {
                    sum += 1;
                }

                // S.M
                if c == 'S'
                    && matrix[i][j + 2] == 'M'
                    && matrix[i + 2][j] == 'S'
                    && matrix[i + 2][j + 2] == 'M'
                {
                    sum += 1
                }

                // M.M
                if c == 'M'
                    && matrix[i][j + 2] == 'M'
                    && matrix[i + 2][j] == 'S'
                    && matrix[i + 2][j + 2] == 'S'
                {
                    sum += 1;
                }

                // S.S
                if c == 'S'
                    && matrix[i][j + 2] == 'S'
                    && matrix[i + 2][j] == 'M'
                    && matrix[i + 2][j + 2] == 'M'
                {
                    sum += 1;
                }
            }
        }
    }

    println!("X-MAS sum: {}", sum);
    assert_eq!(1950, sum);

    Ok(())
}
