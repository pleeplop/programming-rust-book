struct Player {}

impl Player {
    fn location(&self) {
        println!("method()");
    }
}

fn main() {
    println!("Hello, world!");

    let mut i = 0;
    let mut v = Err("an error");
    while let Err(_) = v {
        i += 1;
        if i > 3 {
            v = Ok("success");
        }
    }
    println!("{:?}", v);

    'search:
        for x in 0..3 {
        for y in 0..3 {
            println!("[{}][{}]", x, y);
            if x > y {
                break 'search;
            }
        }
    }

    let p = Player {};
    let rp = &p;
    p.location();
    rp.location();

    let v = Vec::<i32>::with_capacity(32);
    let v = vec![1.0, 2.0, 3.0];
    let sum = sum(&v);
    println!("sum {}", sum);
}

fn sum(v: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for elem in v {
        sum += *elem;
    }
    sum
}
