#[warn(dead_code)]
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
// Two possible alternatives for what a `Broom` could be working on.
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

//==========================================================================
// Example 4 Unit-Like Structs
// A value of such a type occupies no memory, much like the unit type ()
#[warn(unused_variables)]
struct Onesuch;

//==========================================================================
// Example 5 Defining Methods with impl
/// A first-in, first-out queue of characters.
pub struct Queue {
    older: Vec<char>,   // older elements, eldest last.
    younger: Vec<char>, // younger elements, youngest last.
}
impl Queue {
    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    // Push a character onto the back of a queue.
    // &mut self
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    // Pop a character off the front of a queue. Return `Some(c)` if there
    // was a character to pop, or `None` if the queue was empty.
    // &mut self
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            // Bring the elements in younger over to older, and put them in
            // the promised order.
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }

    //&self
    //the method call expression knows which sort of reference to borrow
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    // self by value
    // if a method wants to take ownership of self, it can take self by value:
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

//==========================================================================
// Example 7 Passing Self as Box<Self>, Rc<Self>, Arc<Self>.
use std::rc::Rc;
struct Node {
    tag: String,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new(tag: &str) -> Node {
        Node {
            tag: tag.to_string(),
            children: vec![],
        }
    }

    fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }
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
    let o = Onesuch;

    //==========================================================================
    // Example 5 Defining Methods with impl
    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);

    // the method call expression knows which sort of reference to borrow
    assert!(q.is_empty());
    q.push('☉');
    assert!(!q.is_empty());

    //==========================================================================
    // Example 6 take ownership of self by value
    let mut q2 = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };

    q2.push('P');
    q2.push('D');
    assert_eq!(q2.pop(), Some('P'));
    q2.push('X');

    let (older, younger) = q2.split();
    // q2 is now uninitialized.
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    // Example 7 Passing Self as Box<Self>, Rc<Self>, Arc<Self>.
    let shared_node = Rc::new(Node::new("first"));
    //shared_node.clone().append_to(&mut parent);

    // Example 8
    let mut bq = Box::new(Queue::new());
    // `Queue::push` expects a `&mut Queue`, but `bq` is a `Box<Queue>`.
    // This is fine: Rust borrows a `&mut Queue` from the `Box` for the
    // duration of the call.
    bq.push('■');
}
