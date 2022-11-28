use std::f64::consts;

pub trait GenUnit {
    fn new(tau_phase: f64, sample_rate: f64) -> Self;
    fn sample(&mut self, amp: f64, freq: f64) -> f64;
}

pub trait Visualise {
    fn plot(&self);
}

pub struct GenSine {
    tau_phase: f64,
    radial_step: f64,
}

impl GenUnit for GenSine {
    fn new(tau_phase: f64, sample_rate: f64) -> Self {
        let radial_step = consts::TAU / sample_rate;
        Self {
            tau_phase,
            radial_step,
        }
    }

    fn sample(&mut self, amp: f64, freq: f64) -> f64 {
        let y_amp = amp * (self.tau_phase * freq * consts::TAU).sin();
        self.tau_phase += self.radial_step;
        y_amp
    }
}

impl Visualise for GenSine {
    fn plot(&self) {}
}
