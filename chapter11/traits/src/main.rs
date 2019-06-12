use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, Write};
use std::ops::{Add, Mul};

use serde::Serialize;
use serde_json;

fn main() {
    main_write().unwrap();

    main_serde().unwrap();

    "hello".to_string();
}

fn main_write() -> std::io::Result<()> {
    let mut local_file = File::create("hello.iml")?;
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"hello world\n");

    let mut buf: Vec<u8> = vec![];
//    let writer: Write = buf;
    let writer: &mut Write = &mut buf;

//    let v1 = (0..128).collect();
    let v2 = (0..128).collect::<Vec<i32>>();

    Ok(())
}

fn say_hello(out: &mut Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn say_hello_generic<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n");
    out.flush()
}

fn min<T: Ord>(t1: T, t2: T) -> T {
    if t1 < t2 { t1 } else { t2 }
}

fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {
    unimplemented!();
}

struct DataSet {}

trait Mapper {}

trait Reducer {}

fn run_query<M, R>(data: &DataSet, map: R, reduce: R)
    where M: Mapper + Serialize,
          R: Reducer + Serialize {
    unimplemented!();
}

trait MeasureDistance {}

fn nearest<'t, 'c, P>(target: &'t P, candidate: &'c [P]) -> &'c P
    where P: MeasureDistance {
    unimplemented!();
}

trait Vegetable {}

struct Salad {
    veggies: Vec<Box<Vegetable>>,
}

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct HtmlDocument {}

trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}

impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
        unimplemented!()
    }
}

fn main_serde() -> std::io::Result<()> {
    let mut config = HashMap::new();
    config.insert("key1", "value1");
    config.insert("key2", "value2");


    let writer = File::create("serde.iml")?;
    let mut serializer = serde_json::Serializer::new(writer);
    config.serialize(&mut serializer);

    Ok(())
}

struct CherryTree {}

struct Mammoth {}

pub trait Spliceable {
    fn splice(&self, other: &Self) -> Self;
}

impl Spliceable for CherryTree {
    fn splice(&self, other: &Self) -> Self {
        unimplemented!()
    }
}

//fn splice_anything(left: &Spliceable, right: &Spliceable) {
//    unimplemented!();
//}

trait MegaSpliceable {
    fn splice(&self, other: &MegaSpliceable) -> Box<MegaSpliceable>;
}

fn mega_splice_anything(left: &MegaSpliceable, right: &MegaSpliceable) {
    unimplemented!();
}

trait Visible {
    fn position(&self) -> (u32, u32);
}

struct Direction {}

trait Creature: Visible {
    fn facing(&self) -> Direction;
}

struct Broom {}

impl Visible for Broom {
    fn position(&self) -> (u32, u32) {
        unimplemented!()
    }
}

impl Creature for Broom {
    fn facing(&self) -> Direction {
        unimplemented!()
    }
}

fn string_set_main() {
    let set1 = HashedStringSet::new();
    let set2 = SortedStringSet::new();
}

fn unknown_words<S: StringSet>(document: &Vec<String>, wordlist: &S) -> S {
    let mut unknown = S::new();
    for word in document {
        if !wordlist.contains(word) {
            unknown.add(word);
        }
    }
    unknown
}

trait StringSet {
    fn new() -> Self;
    fn from_slice(strings: &[&str]) -> Self;
    fn contains(&self, string: &str) -> bool;
    fn add(&mut self, string: &str);
}

struct SortedStringSet {}

struct HashedStringSet {}

impl StringSet for SortedStringSet {
    fn new() -> Self {
        unimplemented!()
    }

    fn from_slice(strings: &[&str]) -> Self {
        unimplemented!()
    }

    fn contains(&self, string: &str) -> bool {
        unimplemented!()
    }

    fn add(&mut self, string: &str) {
        unimplemented!()
    }
}

impl StringSet for HashedStringSet {
    fn new() -> Self {
        unimplemented!()
    }

    fn from_slice(strings: &[&str]) -> Self {
        unimplemented!()
    }

    fn contains(&self, string: &str) -> bool {
        unimplemented!()
    }

    fn add(&mut self, string: &str) {
        unimplemented!()
    }
}

trait SizedStringSet {
    fn new() -> Self
        where Self: Sized;
}

fn static_sized_trait_object(wordlist: &SizedStringSet) {
    unimplemented!();
}

fn main_qualified_method_call() {
    let zero = 0;
//    zero.abs();
    i64::abs(zero);
    let line = "hello world";
    let words: Vec<String> = line.split_whitespace()
        .map(<str as ToString>::to_string)    // fully qualified method call
        .collect();
}

fn collect_into_vector<I: Iterator>(iterator: I) -> Vec<I::Item> {
    let mut vec = Vec::new();
    for element in iterator {
        vec.push(element);
    }
    vec
}

fn dump<I: Iterator>(iter: I)
    where
//        I: Iterator<Item=String>,
        I: Iterator,
        I::Item: Debug,
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

struct Complex;

impl Mul<i32> for Complex {
    type Output = Complex;

    fn mul(self, rhs: i32) -> Self::Output {
        unimplemented!()
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

fn main_associated_type() {
    let c1 = Complex {};
    let c2 = c1 * 4;

    let c1 = Complex {};
    let c3 = c1 * c2;
}

fn main_rand() {
    let r = rand::random::<u64>();
    let b = rand::random::<bool>();
}

fn dot<N>(s1: &[N], s2: &[N]) -> N
    where N: Add<Output=N> + Mul<Output=N> + Default + Copy
{
    let mut total = N::default();
    for i in 0..s1.len() {
        total = total + s1[i] * s2[i];
    }
    total
}