use rutie::{RString, AnyObject, Array, Float, Hash};
use cpal::{Stream, StreamConfig};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use ringbuf::RingBuffer;
use std::sync::{Arc, Mutex};
use rustfft::{FftPlanner, num_complex::Complex};

pub struct AudioProcessor {
    audio_buffer: Arc<Mutex<Vec<f32>>>,
    _stream: Stream,
}

impl AudioProcessor {
    pub fn new() -> Self {
        let audio_buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = Arc::clone(&audio_buffer);

        let host = cpal::default_host();
        let device = host.default_input_device().expect("No input device available");
        let config = device.default_input_config().unwrap();

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => Self::run::<f32>(&device, &config.into(), buffer_clone),
            cpal::SampleFormat::I16 => Self::run::<i16>(&device, &config.into(), buffer_clone),
            cpal::SampleFormat::U16 => Self::run::<u16>(&device, &config.into(), buffer_clone),
        }.unwrap();

        Self {
            audio_buffer,
            _stream: stream,
        }
    }

    fn run<T>(device: &cpal::Device, config: &StreamConfig, audio_buffer: Arc<Mutex<Vec<f32>>>) -> Result<Stream, anyhow::Error>
    where
        T: cpal::Sample,
    {
        let ring_buffer = RingBuffer::new(config.sample_rate.0 as usize);
        let (mut producer, mut consumer) = ring_buffer.split();

        let input_data_fn = move |data: &[T], _: &cpal::InputCallbackInfo| {
            for &sample in data {
                let sample: f32 = cpal::Sample::to_f32(&sample);
                producer.push(sample).unwrap();
            }
        };

        let stream = device.build_input_stream(
            config,
            input_data_fn,
            |err| eprintln!("An error occurred on the input stream: {}", err),
        )?;

        stream.play()?;

        let sample_rate = config.sample_rate.0;
        std::thread::spawn(move || {
            loop {
                let mut buffer = audio_buffer.lock().unwrap();
                buffer.clear();
                while let Some(sample) = consumer.pop() {
                    buffer.push(sample);
                }
                if buffer.len() > sample_rate as usize / 20 {  // 50ms of audio
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        Ok(stream)
    }

    pub fn process(input: RString) -> AnyObject {
        let input_str = input.to_string();
        RString::new_utf8(&input_str).into()
    }

    pub fn start_audio_capture() -> AnyObject {
        let _ = Self::new();
        RString::new_utf8("Audio capture started").into()
    }

    pub fn get_audio_data() -> AnyObject {
        let instance = Self::new();
        let buffer = instance.audio_buffer.lock().unwrap();
        let mut ruby_array = Array::new();
        for &sample in buffer.iter() {
            ruby_array.push(Float::new(sample as f64));
        }
        ruby_array.into()
    }

    pub fn analyze_audio() -> AnyObject {
        let instance = Self::new();
        let buffer = instance.audio_buffer.lock().unwrap();

        // Perform FFT
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(buffer.len());
        let mut complex_input: Vec<Complex<f32>> = buffer.iter().map(|&x| Complex::new(x, 0.0)).collect();
        fft.process(&mut complex_input);

        // Find dominant frequency
        let dominant_freq = complex_input.iter()
            .enumerate()
            .max_by_key(|&(_, c)| c.norm() as u32)
            .map(|(i, _)| i as f32 * 44100.0 / buffer.len() as f32)
            .unwrap_or(0.0);

        // Estimate chord (very basic implementation, can be improved)
        let chord = if dominant_freq > 110.0 && dominant_freq < 130.0 {
            "A"
        } else if dominant_freq > 145.0 && dominant_freq < 165.0 {
            "D"
        } else if dominant_freq > 195.0 && dominant_freq < 215.0 {
            "G"
        } else {
            "Unknown"
        };

        let mut result = Hash::new();
        result.store(RString::new_utf8("frequency"), Float::new(dominant_freq as f64));
        result.store(RString::new_utf8("chord"), RString::new_utf8(chord));
        result.into()
    }
}