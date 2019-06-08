extern crate fern_sim;

use fern_sim::Fern;

#[test]
fn should_grow() {
    let mut fern = Fern { size: 1.0, growth_rate: 0.0001 };
    fern.grow();
    assert_eq!(fern.size, 1.0001);
}