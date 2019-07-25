#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::{BTreeSet, HashMap};
use std::default::Default;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::iter::{FromIterator, Peekable};
use std::iter::DoubleEndedIterator;
use std::ops::{Add, Range};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let v = vec![1];
    let iter = v.into_iter();
    let path = Path::new("/tmp/download/fedora.iso");
    let mut iter = path.iter();
    assert_eq!(iter.next(), Some(OsStr::new("/")));
    assert_eq!(iter.next(), Some(OsStr::new("tmp")));
    assert_eq!(iter.next(), Some(OsStr::new("download")));
    assert_eq!(iter.next(), Some(OsStr::new("fedora.iso")));
    assert_eq!(iter.next(), None);

    let mut favourites = BTreeSet::new();
    favourites.insert("Lucy in the sky with Diamonds".to_string());
    favourites.insert("Liebesträume No. 3".to_string());

    let mut iter = favourites.into_iter();
    assert_eq!(iter.next(), Some("Liebesträume No. 3".to_string()));
    assert_eq!(iter.next(), Some("Lucy in the sky with Diamonds".to_string()));
    assert_eq!(iter.next(), None);

    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    assert_eq!(outer, "Eh".to_string());
    assert_eq!(inner, "art".to_string());

    main_iterator_adapter();
    main_peek();
    main_fuse();
    main_rev_iterators();
}

fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += i;
    }
    sum
}

fn triangle_fold(n: i32) -> i32 {
    (1..n + 1).fold(0, |sum, n| sum + n)
}

fn triangle_fold_trait(n: i32) -> i32 {
    (1..n + 1).fold(Default::default(), Add::add)
}

fn dump<T, U>(iterable: T)
    where
        T: IntoIterator<Item=U>,
        U: Debug
{
    for item in iterable.into_iter() {
        println!("{:?}", item);
    }
}

fn main_iterator_adapter() {
    let text = "  ponies  \n   giraffes\niguanas  \nsquid".to_string();
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    let text = "1\nfrond .25  289\n3.1415 estuary\n";
    for number in text.split_whitespace()
        .filter_map(|w| f64::from_str(w).ok()) {
        println!("{:4.2}", number.sqrt());
    }

    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];

    for &city in countries.iter().flat_map(|c| &major_cities[c]) {
        println!("{}", city);
    }

    let iter = (0..10)
        .scan(0, |sum, item| {
            *sum += item;
            if *sum > 10 { None } else { Some(item * item) }
        });
    assert_eq!(iter.collect::<Vec<i32>>(), vec![0, 1, 4, 9, 16]);

    let message = "To: jimb\r\n\
                         From: superego <editor@oreilly.com>\r\n\
                         \r\n\
                         Did you get any writing done today?\r\n\
                         When will you stop wasting time plotting fractals?\r\n";
    let headers: Vec<&str> = message.lines()
        .take_while(|s| *s == "\r\n")
        .collect();
    for header in message.lines().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }

    for body in message.lines().skip_while(|l| !l.is_empty()).skip(1) {
        println!("{}", body);
    }
}

fn main_peek() {
    fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
        where I: Iterator<Item=char>
    {
        let mut n = 0;
        loop {
            match tokens.peek() {
                Some(r) if r.is_digit(10) => {
                    n = n * 10 + r.to_digit(10).unwrap();
                }
                _ => return n
            }
            tokens.next();
        }
    }
    let mut chars = "226153980,1766319049".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153980);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 1766319049);
    assert_eq!(chars.next(), None);
}

fn main_fuse() {
    struct Flaky(bool);

    impl Iterator for Flaky {
        type Item = &'static str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some("totally the last item")
            } else {
                self.0 = true;
                None
            }
        }
    }

    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);
}

fn main_rev_iterators() {
    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();

    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);

    let meals = ["breakfast", "lunch", "dinner"];
    let mut iter = meals.iter().rev();
    assert_eq!(iter.next(), Some(&"dinner"));
    assert_eq!(iter.next(), Some(&"lunch"));
    assert_eq!(iter.next(), Some(&"breakfast"));
    assert_eq!(iter.next(), None);
}

fn main_inspect() {
    let
}