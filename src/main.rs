mod day_1;
mod error;
mod util;

pub use self::error::{Error, Result};

fn main() -> Result<()> {
    println!("\nAdvent of code 2024 (Rust Edition)\n");

    let d1p1 = day_1::part1("./data/day_1.txt")?;
    let d1p2 = day_1::part2("./data/day_1.txt")?;
    println!("Day 1\nPart 1: {d1p1}\tPart 2: {d1p2}\n");

    Ok(())
}
