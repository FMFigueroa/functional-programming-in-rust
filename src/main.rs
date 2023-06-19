// Example 1
#[derive(Debug)]
pub struct GrayscaleMap {
    pub pixels: Vec<u8>,
    pub size: (usize, usize),
}

// Example 1.1
pub fn new_map(_size: (usize, usize), _pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(_pixels.len(), _size.0 * _size.1);
    GrayscaleMap {
        pixels: _pixels,
        size: _size,
    }
    //GrayscaleMap { pixels, size }
}
//==========================================================================
// Example 2 Named-Field Structs.
//In this game, brooms are monsters. You'll see.
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}
/// Two possible alternatives for what a `Broom` could be working on.
#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

// Receive the input Broom by value, taking ownership.
fn chop(b: Broom) -> (Broom, Broom) {
    // Initialize `broom1` mostly from `b`, changing only `height`. Since
    // `String` is not `Copy`, `broom1` takes ownership of `b`'s name.
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };

    // Initialize `broom2` mostly from `broom1`. Since `String` is not
    // `Copy`, we must clone `name` explicitly.
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    // Give each fragment a distinct name.
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

//==========================================================================
// Example 3 Tuple-Like Structs
struct Bounds(usize, usize);

fn tuple_fn(elem0: usize, elem1: usize) -> Bounds {
    let tuple = Bounds(elem0, elem1);

    tuple
}

fn main() {
    // Example 1
    let width = 2;
    let height = 2;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
    //assert_eq!(image.pixels.len(), 1024 * 576);
    //assert_eq!(image.size, (1024, 576));

    // Example 1.1
    let result = new_map(image.size, image.pixels);
    println!("{:?}", result);

    //==========================================================================
    // Example 2 Named-Field Structs.
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey2.health, 100);

    //==========================================================================
    // Example 3 Tuple-Like Structs
    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);
    let tuple = tuple_fn(100, 200);
    assert_eq!((tuple.0, tuple.1), (100, 200));

    //==========================================================================
    // Example 4 Unit-Like Structs
    //A value of such a type occupies no memory, much like the unit type ()
    struct Onesuch;
    let o = Onesuch;
}
