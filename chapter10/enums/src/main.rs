use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    main_ordering();
    main_account();
}

fn main_ordering() {
    assert_eq!(Ordering::Less, compare(2, 3));
}


fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(&self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn rough_time_to_english(rt: &RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, counts) =>
            format!("{} {} ago", counts, units.plural()),
        RoughTime::JustNow =>
            format!("just now"),
        RoughTime::InTheFuture(units, count) =>
            format!("{} {} from now", count, units.plural())
    }
}

enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty =>
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                })),
            BinaryTree::NonEmpty(ref mut node) =>
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
        }
    }
}

#[derive(Debug)]
struct Account {
    name: String,
    language: String,
    id: usize,
}

fn main_account() {
    let account = Account { name: "Doe".to_string(), language: "fr".to_string(), id: 0 };
    match account {
        Account { ref name, ref language, .. } => {
            println!("greet {} {}", &name, &language);
            println!("show_settings {:?}", account)
        }
    }

    let r = &account;
    match r {
        &Account { ref name, .. } => println!("borrow name {}", name)
    }

    let s = "cat".to_string();
    match s.chars().by_ref().into_iter().peekable().peek() {
        Some(&c) => println!("char {}", c),
        None => println!("end of chars"),
    };

    let n = 1;
    match n {
        0 => println!("0"),
        1 | 2 => println!("1 or 2"),
        x @ 3...5 => println!("3..5 = {}", x),
        x if x / 2 == 0 => println!("even"),
        _ => println!("other"),
    }

    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus");
}