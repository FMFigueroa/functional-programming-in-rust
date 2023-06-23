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

    let tensor = vec![
        vec![vec![1, 2, 3], vec![4, 5, 6]],
        vec![vec![7, 8, 9], vec![10, 11, 12]],
    ];

    let flattened: Vec<i32> = tensor
        .into_iter()
        .flat_map(|matrix| matrix.into_iter().flat_map(|row| row.into_iter()))
        .collect();

    for num in flattened {
        println!("{}", num);
    }
    // BTreeMap adapter
    use std::collections::BTreeMap;

    // A table mapping cities to their parks: each value is a vector.
    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    // Build a vector of all parks. `values` gives us an iterator producing
    // vectors, and then `flatten` produces each vector's elements in turn.
    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();
    assert_eq!(
        all_parks,
        vec![
            "Tadasu-no-Mori Forest",
            "Maruyama Koen",
            "Percy Warner Park",
            "Dragon Park",
            "Mt. Tabor Park",
            "Forest Park"
        ]
    );

    assert_eq!(
        vec![None, Some("day"), None, Some("one")]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        vec!["day", "one"]
    );

    let name = "hernandez".to_string();
    let s1 = to_uppercase(&name);
    let s2 = to_uppercase2(&name);
    println!("{}", s1);
    println!("{}", s2);
}

fn to_uppercase(input: &str) -> String {
    input
        .chars()
        .map(char::to_uppercase)
        .flatten() // there's a better way
        .collect()
}

fn to_uppercase2(input: &str) -> String {
    input.chars().flat_map(char::to_uppercase).collect()
}
