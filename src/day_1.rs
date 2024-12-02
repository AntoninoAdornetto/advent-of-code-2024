use std::collections::HashMap;

use crate::error::Result;
use crate::{util, Error};

pub fn part1(path: &str) -> Result<i32> {
    let data = util::read_lines(path)
        .map_err(|err| Error::custom(format!("failed to read input data with error: {}", err)))?;

    let (mut first_set, mut second_set) = parse_pairs(data)?;
    assert_eq!(first_set.len(), second_set.len());
    first_set.sort();
    second_set.sort();

    let sum = sum_difference((first_set, second_set))?;
    Ok(sum)
}

pub fn part2(path: &str) -> Result<i32> {
    let data = util::read_lines(path)
        .map_err(|err| Error::custom(format!("failed to read input data with error: {}", err)))?;

    let (first_set, second_set) = parse_pairs(data)?;
    assert_eq!(first_set.len(), second_set.len());

    let sum = sum_product_frequency((first_set, second_set))?;
    Ok(sum)
}

// Converts line separated pairs of ascii integers, input data for AOC day 1, into-
// a tuple of vector integers. [first_set] represents the first column and [second_set]
// represents the second column.
fn parse_pairs(data: Vec<String>) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut first_set: Vec<i32> = Vec::new();
    let mut second_set: Vec<i32> = Vec::new();

    for line in data {
        let pair: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| {
                x.parse::<i32>().map_err(|_| {
                    Error::custom(format!("failed to convert ASCII value '{}' to i32", x))
                })
            })
            .collect::<Result<Vec<i32>>>()?;

        if pair.len() == 2 {
            pair.into_iter().enumerate().for_each(|(i, n)| match i {
                0 => first_set.push(n),
                1 => second_set.push(n),
                _ => (),
            });
        }
    }

    Ok((first_set, second_set))
}

// calculates the difference between first_set[i] and second_set[i] and sums them up
fn sum_difference((first_set, second_set): (Vec<i32>, Vec<i32>)) -> Result<i32> {
    let sum: i32 = first_set
        .into_iter()
        .enumerate()
        .try_fold(0, |acc, (i, n)| {
            if let Some(m) = second_set.get(i) {
                let diff = (n - m).abs();
                Ok(acc + diff)
            } else {
                Err(Error::custom(format!(
                    "Index {} is out of bounds for second_set of length {}",
                    i,
                    second_set.len()
                )))
            }
        })?;

    Ok(sum)
}

// iterates through the first_set (n) and adds the product of it's frequency in the second_set
fn sum_product_frequency((first_set, second_set): (Vec<i32>, Vec<i32>)) -> Result<i32> {
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for n in second_set.into_iter() {
        frequency_map.entry(n).and_modify(|x| *x += 1).or_insert(1);
    }

    let sum: i32 = first_set.into_iter().fold(0, |acc, n| {
        if let Some(count) = frequency_map.get(&n) {
            acc + (n * count)
        } else {
            acc
        }
    });

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 11;
        let actual = part1("./data/day_1_test_data.txt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let expected = 31;
        let actual = part2("./data/day_1_test_data.txt").unwrap();
        assert_eq!(expected, actual);
    }
}
