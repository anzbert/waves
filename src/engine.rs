use crate::cpal::AudioPlatformCpal;
use cpal::{OutputCallbackInfo, Sample, Stream};
use std::sync::{mpsc::Receiver, Arc, Mutex};

// pub type EngineFn = impl FnMut;

#[derive(Clone)]
pub struct EngineFn {
    pub func: Box<dyn FnMut(f64) -> f64 + Send>,
}

impl EngineFn {
    pub fn new() -> Self {
        let def = |x: f64| 0.;
        Self {
            func: Box::new(def),
        }
    }
}

pub struct Engine {
    pub cpal: AudioPlatformCpal,
    pub stream: Stream,
}

impl Engine {
    pub fn new(func: Arc<Mutex<EngineFn>>) -> Self {
        let cpal = AudioPlatformCpal::new();
        let mut sample_clock: u64 = 0;
        let mut function = EngineFn::new();
        // let sample_time = Duration::from_secs(1).div_f64(config.sample_rate.0 as f64);

        let stream = cpal.build_stream(move |data: &mut [f32], _info: &OutputCallbackInfo| {
            // let output_latency = info
            //     .timestamp()
            //     .playback
            //     .duration_since(&info.timestamp().callback)
            //     .unwrap_or_default();

            match func.try_lock() {
                Ok(func) => function.func = func.func,
                Err(_) => {}
            }

            // Size of provided output buffer for one channel in samples
            assert!(data.len() % cpal.config.channels as usize == 0);
            let buffer_size: usize = data.len() / cpal.config.channels as usize;

            // SYNTH /////////////////////////////////////////////////
            let mut buffer: Vec<f32> = Vec::with_capacity(buffer_size);
            for s in 0..buffer_size {
                buffer.push((function.func)((sample_clock + s as u64) as f64) as f32);
            }

            // ///////////////////////////////////////////////////////

            // Send buffer with same sound output to all channels (equals mono)
            for s in 0..data.len() / cpal.config.channels as usize {
                for c in 0..cpal.config.channels as usize {
                    data[s * cpal.config.channels as usize + c] = Sample::from(&buffer[s]);
                }
            }

            sample_clock += buffer_size as u64;
        });

        Self { cpal, stream }
    }
}
