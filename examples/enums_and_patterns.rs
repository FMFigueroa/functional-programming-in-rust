// cmd: cargo watch -q -c -w examples/ -x 'run --example enums_and_patterns'
use std::mem::size_of;


#[allow(dead_code)]
//#[repr(C)]
enum Ordering {
    Less,
    Equal,
    Greater,
}

#[allow(dead_code)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn main() {
    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);
}
