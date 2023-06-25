fn apply_closure<T, F>(value: T, closure: F)
where
    F: Fn(T),
{
    closure(value);
}

fn main() {
    let data = String::from("Hello, world!");

    let print_value = |value| {
        println!("{}", value);
    };

    apply_closure(data, print_value);
}
