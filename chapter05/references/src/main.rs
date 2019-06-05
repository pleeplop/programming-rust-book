static mut STASH: &i32 = &10;

static CONST: i32 = 100;

fn main() {
    let x = 10;
    let y = 20;
    let mut r = &x;

    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(rx == ry);
    assert!(!std::ptr::eq(rx, ry));

    fn factorial(n: usize) -> usize {
        (1..n + 1).fold(1, |a, b| a * b)
    }
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);

    modify_stash(&CONST);

    static WORTH_POINTING_AT: i32 = 1000;
    let local = 4;
    modify_stash(&WORTH_POINTING_AT);
//    modify_stash(&local);

    let parabola = [9, 4, 1, 0, 1, 4, 9];
    let s = smallest(&parabola);
    assert_eq!(*s, 0);

    let x = 3;
    let s = S { r: &x };

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = U { x: &x, y: &y };
            r = s.x;
        };
    };
    let x = 1;
    let u = U { x: &2, y: &2 };
    let s = sum_r_xy(&x, u);
    println!("sum_r_xy {}", s);

    let v = vec![4, 8, 19, 27, 34, 10];
    let r = &v;
    let aside = v;  // move vector to aside
//    r[0];

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = vec![0.0, -1.0];
    extend(&mut wave, &head);
    extend(&mut wave, &tail);
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);
    extend(&mut wave, &wave);

    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0;         // ok: reborrowing shared as shared
//    let m1 = &mut r.1;
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for x in slice {
        vec.push(*x);
    }
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for e in self.elements.iter() {
            if e.starts_with(prefix) {
                return Some(e);
            }
        }
        None
    }
}

fn sum_r_xy(r: &i32, u: U) -> i32 {
    r + u.x + u.y
}

struct T<'a> {
    s: S<'a>,
}

struct S<'a> {
    r: &'a i32,
}

struct U<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn modify_stash(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in v {
        if *r < *s {
            s = r;
        }
    }
    s
}
