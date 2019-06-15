#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::fmt::Display;
use std::ops::{Deref, DerefMut, Index};

use rand;

fn main() {
    main_rc_box();
    main_clone();
    main_deref();
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
