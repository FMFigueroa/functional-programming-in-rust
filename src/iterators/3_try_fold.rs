use std::{error::Error, io::prelude::*, str::FromStr};

// cmd: cargo watch -q -c -w examples/ -x 'run --example 3_try_fold'

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let sum = stdin
        .lock()
        .lines()
        .try_fold(0, |sum, line| -> Result<u64, Box<dyn Error>> {
            Ok(sum + u64::from_str(&line?.trim())?)
        })?;
    println!("{}", sum);
    Ok(())
}
