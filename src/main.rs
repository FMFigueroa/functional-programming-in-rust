fn main() {
    let text = " ponies \n  giraffes \n iguanas\nsquid ".to_string();

    // Imperative Programming
    let mut v = vec![];
    for line in text.lines() {
        let line = line.trim();
        if line != "iguanas" {
            v.push(line);
        }
    }
    assert_eq!(v, ["ponies", "giraffes", "squid"]);
    println!("{:?}", v);

    //========================================================
    // Functional Programming
    // Map Adapter
    let v1: Vec<&str> = text.lines().map(str::trim).collect();
    assert_eq!(v1, ["ponies", "giraffes", "iguanas", "squid"]);
    println!("{:?}", v1);

    // Filter Adapter
    let v2: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v2, ["ponies", "giraffes", "squid"]);
    println!("{:?}", v2);
}
