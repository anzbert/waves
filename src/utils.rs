use std::f64::consts;

pub fn phase_from_sample_clock(sample_clock: u64, sample_rate: f64) -> f64 {
    let radial_step = consts::TAU / sample_rate;
    sample_clock as f64 % radial_step
}

pub fn time_from_sample_clock(sample_clock: u64, sample_rate: f64) -> f64 {
    sample_clock as f64 / sample_rate
}
