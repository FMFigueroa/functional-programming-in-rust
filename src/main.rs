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
    // map adapter
    let v1: Vec<&str> = text.lines().map(str::trim).collect();
    assert_eq!(v1, ["ponies", "giraffes", "iguanas", "squid"]);
    println!("{:?}", v1);

    // filter adapter
    let v2: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v2, ["ponies", "giraffes", "squid"]);
    println!("{:?}", v2);

    // filter_map adapter
    use std::str::FromStr;

    let text = "1\nfrond .25  289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
    {
        println!("{:4.2}", number.sqrt());
    }

    // filter_map adapter
    for number in text
        .split_whitespace()
        .map(|w| f64::from_str(w))
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
    {
        println!("{:4.2}", number.sqrt());
    }

    // filter_map adapter without for cycle
    let text2 = "1\nfrond .25  289\n3.1415 estuary\n";
    let v3: Vec<f64> = text2
        .split_whitespace()
        .filter_map(|s| f64::from_str(s).ok())
        .map(|x| x.sqrt())
        .collect();

    println!("{:.2?}", v3);

    // flat_map adapter
    use std::collections::HashMap;

    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];

    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }
}
