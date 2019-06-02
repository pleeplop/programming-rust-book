fn main() {
    let n = 8u8;
    let ascii_code = b'A';
    let ascii_char = ascii_code as char;

    assert_eq!(0b1011_0100u8.count_ones(), 4);

    let infinity = std::f32::INFINITY;
    println!("ascii_code {} ascii_char {} infinity {}", ascii_code, ascii_char, infinity);

    let v = vec![1.0, 2.0, 3.0];
    print(&v[1..3]);
    let s = r###"
    pif paf pouf ' " #
    "###;
    println!("{}", s);
    let o = None;
}

fn print(n: &[f64]) {
    for x in n {
        println!("x");
    }
}