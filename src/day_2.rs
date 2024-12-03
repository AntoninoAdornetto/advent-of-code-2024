use std::cmp::Ordering;

use crate::{error::Result, util};

pub fn part1(path: &str) -> Result<i32> {
    let data = util::read_lines(path)?;
    let levels = parse_levels(data)?;
    let count = count_safe(levels)?;
    Ok(count)
}

pub fn part2(path: &str) -> Result<i32> {
    let data = util::read_lines(path)?;
    let _ = parse_levels(data)?;
    Ok(1)
}

fn parse_levels(data: Vec<String>) -> Result<Vec<Vec<i32>>> {
    let levels = data
        .iter()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    Ok(levels)
}

// NOTE: sub optimal implementation. Quickly pieced this together off 3 hours of sleep and my brain
// hurts. I'll revise the approach/implementation when I solve part 2
fn count_safe(levels: Vec<Vec<i32>>) -> Result<i32> {
    let mut sum = 0;

    for level in levels {
        let mut ok = false;
        let mut order = 0;

        for i in 0..level.len() - 1 {
            let left = level[i];
            let right = level[i + 1];

            if order == 0 {
                match left.cmp(&right) {
                    Ordering::Greater => {
                        order = -1;
                    }
                    Ordering::Less => {
                        order = 1;
                    }
                    _ => (),
                }
            }

            match order {
                1 => {
                    let delta = right - left;
                    ok = (left < right) && (delta > 0 && delta < 4);
                }
                -1 => {
                    let delta = left - right;
                    ok = (left > right) && (delta > 0 && delta < 4);
                }
                _ => {
                    ok = false;
                }
            }

            if !ok {
                break;
            }
        }

        if ok {
            sum += 1;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let expected = 2;
        let actual = part1("./data/day_2_test_data.txt").unwrap();
        assert_eq!(expected, actual);
    }
}
