// Example 1 =====================================
fn bar<F, T>(f: F, x: T) -> T
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
use std::{collections::HashMap, fmt::Debug};
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

    println!("{:?}", complex_values);

    // Example 4.1
    fn foo<F>(g: F, x: u32) -> u32
    where
        F: Fn(u32) -> u32,
    {
        let result = g(g(x));
        result + x
    }
    let res_foo = foo(|n| n * 2, 5);
    println!("Res_Foo: {}", res_foo);

    // Example 5
    let result: i32 = (0..10)
        .map(|x| x + 1)
        .inspect(|x| println!("Inspecting: {}", x))
        .filter(|x| x % 2 == 0)
        .filter_map(|x| if x < 5 { Some(x) } else { None })
        .fold(0, |acc, x| acc + x);

    println!("Result: {}", result);

    // Example 6
    let names = vec!["Alice", "Bob", "Charlie"];
    let lengths: Vec<usize> = names.iter().map(|name| name.len()).collect();
    println!("Result: {:?}", lengths);

    let total = lengths.iter().fold(0, |acc, &lengths| acc + lengths);
    println!("Result: {:?}", total);

    // Example 7 Fn()
    let mut bucket = HashMap::new();
    bucket.insert("Jhon", 20);
    bucket.insert("patrick", 35);
    bucket.insert("jesse", 18);
    let map_bucket = || {
        for (key, value) in &bucket {
            println!("name:{:?} age:{:?}", key, value);
        }
    };
    fn call<F>(closure: F)
    where
        F: Fn(),
    {
        closure();
    }
    call(map_bucket);

    // Example 8 FnMut()
    fn call_twice<F>(mut closure: F)
    where
        F: FnMut(),
    {
        closure();
        closure();
        closure();
        closure();
        closure();
    }

    let mut i = 0;
    let incr = || {
        i += 1; // incr borrows a mut reference to i
        println!("Ding! i is now: {}", i);
    };
    call_twice(incr);

    // Example 9
    let y = 10;
    let add_y = |x| x + y;
    let copy_of_add_y = add_y;
    // This closure is `Copy`, so...
    assert_eq!(add_y(copy_of_add_y(22)), 42); // ... we can call both.

    // Example 10
    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("Alfred");
    greet.clone()("Bruce");
}
