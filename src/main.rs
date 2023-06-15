// Example 1 =====================================
fn foo<F, T>(f: F, x: T) -> T
where
    //  Trait bound Fn
    F: Fn(T) -> T,
{
    f(x)
}

// Example 2 =====================================
fn map<T, U, F>(collection: &[T], f: F) -> Vec<U>
where
    //  Trait bound Fn
    F: Fn(&T) -> U,
{
    collection.iter().map(f).collect()
}

// Example 3 ========================================
use std::fmt::Debug;
fn print_value<T: Debug>(value: T) {
    println!("Value: {:?}", value);
}

fn process_data<F, T>(func: F, data: T)
where
    // Traits bound:
    T: Debug,
    F: Fn(T),
{
    // processing logic
    func(data);
}

// Example 3.1 ========================================
fn apply_closure<F, T>(closure: F, value: T)
where
    // Traits bound:
    F: Fn(T),
{
    // processing logic
    closure(value);
}

fn main() {
    // Example 1
    let result = foo(|x| x * x, 5);
    println!("{}", result);

    // Example 2
    let numbers = vec![1, 2, 3, 4, 5];
    // Map: Aplicar una función a cada elemento de la colección
    let squared_numbers: Vec<i32> = map(&numbers, |x| x * x);
    println!("Squared numbers: {:?}", squared_numbers);

    // Example 3
    let data = String::from("hello world");
    process_data(print_value, &data);

    // Example 3.1
    let print_value = |value| {
        println!("Value: {:?}", value);
    };
    apply_closure(print_value, data);
}
