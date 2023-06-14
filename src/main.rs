// Example 1
fn foo<F, T>(f: F, x: T) -> T
where
    //  Trait bound Fn
    F: Fn(T) -> T,
{ 
    f(x)
}

// Example 2
fn map<T, U, F>(collection: &[T], f: F) -> Vec<U>
where
    //  Trait bound Fn
    F: Fn(&T) -> U,
{
    collection.iter().map(f).collect()
}

fn main() {
    // Example 1
    let result = foo(|x| x * x, 2);
    println!("{}", result);

    // Example 2
    let numbers = vec![1, 2, 3, 4, 5];
    // Map: Aplicar una función a cada elemento de la colección
    let squared_numbers: Vec<i32> = map(&numbers, |x| x * x);
    println!("Squared numbers: {:?}", squared_numbers);
}
