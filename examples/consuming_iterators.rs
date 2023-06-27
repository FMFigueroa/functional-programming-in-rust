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
}

// sum and product method
fn triangle(n: u64) -> u64 {
    (1..=n).sum()
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}
