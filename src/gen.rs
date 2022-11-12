pub trait GenUnit {
    fn new(phase: f64) -> Self;
    fn sample(amp: f64) -> f64;
}

pub struct GenSine {}

impl GenUnit for GenSine {
    fn new(phase: f64) -> Self {
        Self {}
    }

    fn sample(amp: f64) -> f64 {
        todo!()
    }
}
