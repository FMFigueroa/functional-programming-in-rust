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

    // flatten adapter
    assert_eq!(
        vec![None, Some("day"), None, Some("one")]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        vec!["day", "one"]
    );

    // faltten adapter
    let name = "hernandez".to_string();
    let s1 = to_uppercase_fn(&name);
    println!("{}", s1);

    // version 1
    let nombres: String = "hernandez, carlos,  ramon, pedro".to_string();
    let s2: Vec<String> = nombres
        .split(", ")
        .map(|nombre| nombre.trim().to_uppercase())
        .collect();
    println!("{:?}", s2);

    // version 2
    let s2: Vec<String> = nombres
        .split(", ")
        .map(|nombre| {
            nombre
                .trim()
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().next().unwrap()
                    } else {
                        c
                    }
                })
                .collect::<String>()
        })
        .collect();
    println!("{:?}", s2);

    // version 3
    let s3: Vec<String> = nombres
        .split(", ")
        .map(|nombre| {
            let mut nombre = nombre.trim().to_lowercase().to_string();
            if let Some(first_char) = nombre.chars().next() {
                let uppercased = first_char.to_uppercase();
                nombre.replace_range(..1, &uppercased.to_string());
            }
            nombre
        })
        .collect();
    println!("{:?}", s3);

    // version 4
    let s4: Vec<String> = nombres
        .split(", ")
        .map(|nombre| {
            nombre
                .trim()
                .chars()
                .enumerate()
                .map(|(i, c)| match i {
                    0 => c.to_uppercase().collect::<String>(),
                    _ => c.to_string(),
                })
                .collect::<String>()
        })
        .collect();
    println!("{:?}", s4);

    // take and take_while adapter
    let msn = "To: jimb\r\n\
    From: superego <editor@oreilly.com>\r\n\
    \r\n\
    Did you get any writing done today?\r\n\
    When will you stop wasting time plotting fractals?\r\n";

    // take and take_while adapter
    for header in msn.lines().take_while(|line| !line.is_empty()) {
        println!("{}", header);
    }

    // skip and skip_while adapter
    for body in msn.lines().skip_while(|l| !l.is_empty()).skip(1) {
        println!("{}", body);
    }

    // peekable adapter
    let mut chars = "226153980,1766319049".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153980);
    // Look, `parse_number` didn't consume the comma! So we will.
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 1766319049);
    assert_eq!(chars.next(), None);

    // fuse adapter
    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);

    // trait DoubleEndedIterator | next and next_back
    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();

    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));

    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    // rev adapter
    let meals = ["breakfast", "lunch", "dinner"];

    let mut iter = meals.iter().rev();
    assert_eq!(iter.next(), Some(&"dinner"));
    assert_eq!(iter.next(), Some(&"lunch"));
    assert_eq!(iter.next(), Some(&"breakfast"));
    assert_eq!(iter.next(), None);

    // inspect adapter
    let upper_case: String = "große"
        .chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!(" after: {:?}", c))
        .collect();
    assert_eq!(upper_case, "GROSSE");
    println!("result of upper_case:{:?}", upper_case);

    // chain adapter
    let v_chain: Vec<i32> = (1..4).chain(vec![20, 30, 40]).collect();
    assert_eq!(v_chain, [1, 2, 3, 20, 30, 40]);
    // chain adapter rev
    let v_chain: Vec<i32> = (1..4).chain(vec![20, 30, 40]).rev().collect();
    assert_eq!(v_chain, [40, 30, 20, 3, 2, 1]);
    // zip adapter
    let v_zip: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v_zip, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);

    use std::iter::repeat;
    let endings = vec!["once", "twice", "chicken soup with rice"];
    let rhyme: Vec<_> = repeat("going").zip(endings).collect();
    assert_eq!(
        rhyme,
        vec![
            ("going", "once"),
            ("going", "twice"),
            ("going", "chicken soup with rice")
        ]
    );
    // by_ref adapter
    let message = "To: jimb\r\n\
    From: id\r\n\
	\r\n\
	Oooooh, donuts!!\r\n";

    let mut lines = message.lines();
    println!("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }
    println!("\nBody:");
    for body in lines {
        println!("{}", body);
    }
    // cloned, copied adapter
}

// faltten adapter
fn to_uppercase_fn(input: &str) -> String {
    input
        .chars()
        .map(char::to_uppercase)
        .flatten() // there's a better way
        .collect()
}

// peekable adapter
use std::iter::Peekable;

fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
where
    I: Iterator<Item = char>,
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n,
        }
        tokens.next();
    }
}

// fuse adapter
struct Flaky(bool);

impl Iterator for Flaky {
    type Item = &'static str;

    // flip flop true or false
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            self.0 = false;
            Some("totally the last item")
        } else {
            self.0 = true; // D'oh!
            None
        }
    }
}
