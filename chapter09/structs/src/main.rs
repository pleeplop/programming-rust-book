use std::cell::{Cell, RefCell};

/// A rectangle og eight-bit grayscale pixels
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

impl GrayscaleMap {
    pub fn new(pixels: Vec<u8>, size: (usize, usize)) -> GrayscaleMap {
        assert_eq!(pixels.len(), size.0 * size.1);
        GrayscaleMap { pixels, size }
    }
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }

struct Bounds(usize, usize);

fn main() {
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap::new(vec![0; width * height], (width, height));

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };
    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.health, 100);
    assert_eq!(hokey2.name, "Hokey II");

    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);

    main_queue();
    main_lifetime();
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom { height: b.height / 2, ..b };
    let mut broom2 = Broom { name: broom1.name.clone(), ..broom1 };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

fn main_queue() {
    let mut q = Queue::new();

    assert!(q.is_empty());
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert!(q.is_empty());
    assert_eq!(q.pop(), None);

    // turbo fish
    let mut q = Queue::<char>::new();

    q.push('0');
    q.push('1');
    let (older, younger) = q.split();
    assert!(older.is_empty());
    assert_eq!(younger, vec!['0', '1']);
}

fn main_lifetime() {
    let s = [0, -3, 0, 15, 48];
    let e = Extrema::find_extrema(&s);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}

/// A last-in, first-out queue of characters.
pub struct Queue<T> {
    // younger elements, youngest last.
    younger: Vec<T>,
    // older elements, eldest last.
    older: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { younger: Vec::new(), older: Vec::new() }
    }

    /// Push a character onto the back of the queue.
    pub fn push(&mut self, c: T) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.younger.is_empty() && self.older.is_empty()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

struct Extrema<'a> {
    greatest: &'a i32,
    least: &'a i32,
}

impl<'a> Extrema<'a> {
    pub fn find_extrema(slice: &'a [i32]) -> Extrema<'a> {
        let mut greatest = &slice[0];
        let mut least = &slice[0];

        for i in 1..slice.len() {
            if slice[i] > *greatest {
                greatest = &slice[i];
            }
            if slice[i] < *least {
                least = &slice[i];
            }
        }

        Extrema { greatest, least }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

pub struct SpiderRobot {
    hardware_error_count: Cell<u32>,
}

impl SpiderRobot {
    /// Increase the error count by 1.
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    /// True if any hardware errors have been reported.
    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }
}

fn main_cell() {
    let ref_cell = RefCell::new("hello".to_string());
    {
        let r = ref_cell.borrow();
        let count = r.len();
        assert_eq!(count, 5);
    }
    {
        let mut w = ref_cell.borrow_mut();
        w.push_str(" world");
        assert_eq!(*w, "hello world".to_string());
    }
}