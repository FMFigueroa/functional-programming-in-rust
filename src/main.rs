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

    // Example 4
    let complex_values: Vec<_> = (0..10)
        .map(|x| {
            // Definición de función en el cierre
            fn complex_equation(y: i32) -> i32 {
                // Ecuación compleja
                y.pow(2) + 2 * y + 1
            }

            let result = complex_equation(x); // Variable vinculada

            result
        })
        .collect();

    print!("{:?}", complex_values);

    // Example 5
    let result: i32 = (0..10)
        .map(|x| x + 1)
        .inspect(|x| println!("Inspecting: {}", x))
        .filter(|x| x % 2 == 0)
        .filter_map(|x| if x < 5 { Some(x) } else { None })
        .fold(0, |acc, x| acc + x);

    println!("Result: {}", result);
    
    // Exmaple 6
    let names = vec!["Alice", "Bob", "Charlie"];
    let lengths: Vec<usize> = names.iter().map(|name| name.len()).collect();
    println!("Result: {:?}", lengths);

    let total = lengths.iter().fold(0, |acc, &lengths| acc + lengths);
    println!("Result: {:?}", total);
}
