#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::{Deref, DerefMut, Index};
use std::path::Path;

use rand;

fn main() {
    main_rc_box();
    main_clone();
    main_deref();
    main_default();
}

struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

fn main_appelation() {
    let mut a = Appellation {
        name: "Zeus".to_string(),
        nicknames: vec!["cloud collector".to_string(), "king of the gods".to_string()],
    };
    println!("before assignement");
    a = Appellation { name: "Hera".to_string(), nicknames: vec![] };
    println!("at the end of block");
}

fn flag_drop() {
    let p;
    {
        let q = Appellation {
            name: "Cardamine hirsuta".to_string(),
            nicknames: vec!["shotweed".to_string(),
                            "bittercress".to_string()],
        };
        if rand::random::<bool>() {
            println!("p = q");
            p = q;
        }
    }
    println!("Sproing! What was that?");
}

struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T,
}

fn main_rc_box() {
    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string(),
    };
    let boxed_displayable: &RcBox<Display> = &boxed_lunch;
    display(&boxed_lunch);
}

fn display(boxed: &RcBox<Display>) {
    println!("{}", &boxed.value);
}

fn main_clone() {
    let mut s = "Hello".to_string();
    let v = "Hey".to_string();
    s.clone_from(&v);
}

#[derive(Clone, Copy, Debug)]
struct MyType(u32);

fn main_copy() {
    let t = MyType(2);
    let v = t;
    println!("{:?} {:?}", t, v);
}

struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

fn main_deref() {
    let mut s = Selector { elements: vec!['x', 'y', 'z'], current: 2 };
    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());
    *s = 'w';
    assert_eq!(s.elements, ['x', 'y', 'w']);

    let s = Selector { elements: vec!["one", "two", "three"], current: 2 };
    show_it(&s as &str)
}

fn show_it<T: Display>(thing: T) {
    println!("{}", thing);
}

fn main_default() {
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>)
        = squares.iter().partition(|&n| n & (n - 1) == 0);
    assert_eq!(powers_of_two.len(), 3);
    assert_eq!(impure.len(), 4);

    let (upper, lower): (String, String)
        = "Great Teacher Onizuka".chars().partition(|&c| c.is_uppercase());
    assert_eq!(upper, "GTO");
    assert_eq!(lower, "reat eacher nizuka");
}

fn main_as_ref() {
    let dot_emacs = std::fs::File::open("~/.emacs");
}

fn main_borrow() {
    let mut map = HashMap::new();
    let k1 = "k1".to_string();
    map.insert(k1, 1);
    let x = "k1";
    let v1 = map.get(x);
}

fn main_from_into() {
    use std::net::Ipv4Addr;
    fn ping<A>(address: A) -> std::io::Result<bool>
        where A: Into<Ipv4Addr> {
        let ipv4: Ipv4Addr = address.into();
        unimplemented!();
    }
    println!("{:?}", ping(Ipv4Addr::new(23, 21, 68, 141)));
    println!("{:?}", ping([23, 21, 68, 141]));
    println!("{:?}", ping(0xd076eb94_u32));

    let addr1 = Ipv4Addr::from([23, 21, 68, 141]);
    let addr2 = Ipv4Addr::from(0xd076eb94_u32);

    let heap_allocated = String::from("static str");
}

fn main_to_owned() {
    let str = "str static";
    let string = str.to_owned();

    let slice = [1, 2, 3];
    let vec = slice.to_owned();
}

fn main_cow() {
    enum Error {
        StackOverflow,
        FileNotFound(&'static Path),
    }

    use std::path::PathBuf;
    fn describe(error: &Error) -> Cow<'static, str> {
        match error {
            Error::StackOverflow => "out of memory".into(),
            Error::FileNotFound(ref path) => format!("file not found: {}", path.display()).into(),
        }
    }
}