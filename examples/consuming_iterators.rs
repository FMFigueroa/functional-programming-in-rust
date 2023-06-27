// cmd: cargo watch -q -c -w examples/ -x 'run --example consuming_iterators'
fn main() {
    // count method
    /* let mut input = String::new();
    loop {
        std::io::stdin()
            .read_line(&mut input)
            .expect("input read error");
        println!("{}", input.lines().count());
    } */

    // sum and product methods
    assert_eq!(triangle(20), 210);
    assert_eq!(factorial(20), 2432902008176640000);

    // min and max methods
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));

    // max_by, min_by methods
    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));
    // let numbers = [1.0, 4.0, std::f64::NAN, 2.0];
    // assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0)); // panics

    // max_by_key, min_by_key methods
    use std::collections::HashMap;

    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 7_762);
    populations.insert("Boring", 2);
    populations.insert("The Dalles", 15_340);

    assert_eq!(
        populations.iter().max_by_key(|&(_name, pop)| pop),
        Some((&"Portland", &583_776))
    );
    assert_eq!(
        populations.iter().min_by_key(|&(_name, pop)| pop),
        Some((&"Boring", &2))
    );

    // Comparing Item Sequences
    let packed = "Helen of Troy";
    let spaced = "Helen   of    Troy";
    let obscure = "Helen of Sandusky"; // nice person, just not famous
    assert!(packed != spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));
    // This is true because ' ' < 'o'.
    assert!(spaced < obscure);
    // This is true because 'Troy' > 'Sandusky'.
    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));
}

// sum and product methods
fn triangle(n: u64) -> u64 {
    (1..=n).sum()
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

// max_by,  min_by methods
use std::cmp::Ordering;

// Compare two f64 values. Panic if given a NaN.
fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}
