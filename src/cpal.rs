use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Device, OutputCallbackInfo, Sample, SampleFormat, SupportedStreamConfig};
use cpal::{Stream, StreamConfig};
use std::time::Duration;

/// Audio Buffer size
const BUFFER_SIZE: u32 = 512;

/// Handles Multiplatform audio output with 'cpal'.
pub struct AudioPlatformCpal {
    pub config: StreamConfig,
    device: Device,
    supported_config: SupportedStreamConfig,
    pub info: String,
}

impl AudioPlatformCpal {
    pub fn new() -> Self {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("No output device available");

        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("Error while querying configs");

        let first_supported_config = supported_configs_range
            .next()
            .expect("No supported config?!")
            .with_max_sample_rate();

        let mut config = first_supported_config.config();
        config.buffer_size = BufferSize::Fixed(BUFFER_SIZE);

        let mut info = String::new();

        info.push_str(
            format!(
                "SAMPLE RATE: {} (SampleFormat::{:?})\n",
                config.sample_rate.0,
                first_supported_config.sample_format()
            )
            .as_str(),
        );

        info.push_str(
            format!(
                "DEVICE NAME: {}",
                device.name().expect("Could not get device name.\n"),
            )
            .as_str(),
        );

        info.push_str(
            format!(
                "BUFFER SIZE: {} samples, {:.2} ms (Supported {:?})\n",
                BUFFER_SIZE,
                BUFFER_SIZE as f64 * 1000. / config.sample_rate.0 as f64,
                first_supported_config.buffer_size()
            )
            .as_str(),
        );

        let channel_cfg = match config.channels {
            1 => "(Mono)",
            2 => "(Stereo)",
            _ => "",
        };

        info.push_str(format!("OUTPUT CHANNELS: {} {}", config.channels, channel_cfg).as_str());

        Self {
            device,
            config,
            supported_config: first_supported_config,
            info,
        }
    }

    /// Build an Audio Stream in the correct format with a provided engine callback function
    pub fn build_stream<T: Sample>(
        &self,
        callback: (impl FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static),
    ) -> Stream {
        let err_fn = |err| panic!("An error occurred on the output audio stream: {}", err);

        let stream = match self.supported_config.sample_format() {
            SampleFormat::F32 => self
                .device
                .build_output_stream(&self.config, callback, err_fn),
            SampleFormat::I16 => self
                .device
                .build_output_stream(&self.config, callback, err_fn),
            SampleFormat::U16 => self
                .device
                .build_output_stream(&self.config, callback, err_fn),
        }
        .unwrap();

        stream.play().unwrap();

        stream
    }
}
pub struct EngineCallbackParams {
    buffer_size: usize,
    sample_rate: u64,
    output_latency: Duration,
    sample_time: Duration,
    sample_count: u64,
}
