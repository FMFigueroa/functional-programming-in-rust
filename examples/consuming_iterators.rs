fn main() {
    // count method
    /* let mut input = String::new();
    loop {
        std::io::stdin()
            .read_line(&mut input)
            .expect("input read error");
        println!("{}", input.lines().count());
    } */

    // sum and product method
    assert_eq!(triangle(20), 210);
    assert_eq!(factorial(20), 2432902008176640000);

    // min and max method
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));

    //max_by.. min_by method
    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));
    let numbers = [1.0, 4.0, NAN, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0)); // panics
}

// sum and product method
fn triangle(n: u64) -> u64 {
    (1..=n).sum()
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

//max_by.. min_by method
use std::cmp::Ordering;
use std::f64::NAN;
// Compare two f64 values. Panic if given a NaN.
fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}
