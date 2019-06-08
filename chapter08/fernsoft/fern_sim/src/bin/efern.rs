extern crate fern_sim;

use fern_sim::{Fern, run_simulation};

fn main() {
    let mut fern = Fern { size: 1.0, growth_rate: 0.001 };
    run_simulation(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}

#[cfg(test)]
mod tests {
    #[test]
    fn math_works() {
        let x: i32 = 1;
        assert!(x.is_positive());
        assert_eq!(x + 1, 2);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn should_panic_divide_by_zero_error() {
        let x = 0;
        let _ = 1 / x;
    }

    fn roughly_equals(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equals(PI.sin(), 0.0));
    }
}