use std::ops::Range;
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

pub struct Terrarium {}

impl Terrarium {
    pub fn new() -> Terrarium {
        Terrarium {}
    }

    /// Let the sun shine in and run the simulation for a given amount of time.
    ///
    ///     # use fern_sim::Terrarium;
    ///     # use std::time::Duration;
    ///     # let mut tm = Terrarium::new();
    ///     tm.apply_sunlight(Duration::from_secs(60));
    ///
    pub fn apply_sunlight(&mut self, time: Duration) {}
}

impl Fern {
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}

/// Return true if two range overlap
///
///     assert_eq!(fern_sim::overlap(0..7, 3..10), true);
///
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end &&
        r1.start < r2.end && r2.start < r1.end
}

pub struct Session {}

impl Session {
    /// Upload all terrariums to the online gallery
    ///
    /// ```no_run
    /// let mut session = fern_sim::connect();
    /// session.upload_all();
    /// ```
    pub fn upload_all(&mut self) {}
}

pub fn connect() -> Session {
    Session {}
}
