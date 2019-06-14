#![allow(unused_variables)]

use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Index, IndexMut, Neg};

fn main() {
    assert_eq!(Add::add(1, 2), 3);
    let c1 = Complex { re: -1.0, im: 1.0 };
    let c2 = Complex { re: 1.0, im: -1.0 };
    let c3 = c1 + c2;

    main_interval();
    main_index();
}

pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<L, R, O> Add<Complex<R>> for Complex<L>
    where L: Add<R, Output=O>
{
    type Output = Complex<O>;

    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl<T, O> Neg for Complex<T>
    where T: Neg<Output=O>
{
    type Output = Complex<O>;

    fn neg(self) -> Self::Output {
        Complex { re: -self.re, im: -self.im }
    }
}

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> PartialEq for Complex<T>
    where T: PartialEq<T>
{
    fn eq(&self, other: &Self) -> bool {
        self.im == other.im && self.re == other.re
    }
}

impl<T: Eq> Eq for Complex<T> {}

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T,
    upper: T,
}

impl<T: PartialOrd> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

fn main_interval() {
    assert!(Interval { lower: 1, upper: 2 } < Interval { lower: 3, upper: 4 });
    assert!(!(Interval { lower: 1, upper: 3 } < Interval { lower: 2, upper: 4 }))
}

fn main_index() {
    let mut map = HashMap::new();
    map.insert("k1", 1);
    map.insert("k2", 3);
    map.insert("k3", 3);

    assert_eq!(map["k1"], 1);
    assert_eq!(*map.index("k1"), 1);

    let mut v = vec!["s1".to_string(), "s2".to_string()];
    v[0].push_str("append");
    (*v.index_mut(1)).push_str("append");
}

struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P: Default + Copy> Index<usize> for Image<P> {
    type Output = [P];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P: Default + Copy> IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = self.width * row;
        &mut self.pixels[start..start + self.width]
    }
}