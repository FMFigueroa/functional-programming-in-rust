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

    // any and all methods
    let id = "Iterator";
    assert!(id.chars().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));

    // position, rposition, and ExactSizeIterator methods
    let text = "Xerxes";
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'z'), None);

    // rposition
    let bytes = b"Xerxes";
    assert_eq!(bytes.iter().rposition(|&c| c == b'e'), Some(4));
    assert_eq!(bytes.iter().rposition(|&c| c == b'X'), Some(0));

    // fold and rfold methods
    let a = [5, 6, 7, 8, 9, 10];
    // count
    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);
    // sum
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45);
    // product
    assert_eq!(a.iter().fold(1, |n, i| n * i), 151200);
    // max
    assert_eq!(a.iter().cloned().fold(i32::min_value(), std::cmp::max), 10);

    // string fold
    let a = [
        "Pack", "my", "box", "with", "five", "dozen", "liquor", "jugs",
    ];
    // See also: the `join` method on slices, which won't
    // give you that extra space at the end.
    let pangram = a.iter().fold(String::new(), |s, w| s + w + " ");
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs ");

    // string rfold
    let weird_pangram = a.iter().rfold(String::new(), |s, w| s + w + " ");
    assert_eq!(weird_pangram, "jugs liquor dozen five with box my Pack ");

    // nth, nth_back methods
    let mut squares = (0..10).map(|i| i * i);
    assert_eq!(squares.nth(1), Some(1)); // index 1
    assert_eq!(squares.nth(0), Some(4)); // index 2
    assert_eq!(squares.nth(2), Some(25)); // index 5
    assert_eq!(squares.nth(1), Some(49)); // index 7
    assert_eq!(squares.nth(0), Some(64)); // index 8
    assert_eq!(squares.nth(0), Some(81)); // index 9
    assert_eq!(squares.nth(10), None); // index 10

    // last method
    let squares = (0..10).map(|i| i * i);
    assert_eq!(squares.last(), Some(81));
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
