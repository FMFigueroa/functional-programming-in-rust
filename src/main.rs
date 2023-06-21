fn main() {
    let text = " ponies \n  giraffes \n iguanas\nsquid ".to_string();
    let v: Vec<&str> = text.lines().map(str::trim).collect();

    assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);
    println!("{:?}", v);

    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);
     println!("{:?}", v);
}
