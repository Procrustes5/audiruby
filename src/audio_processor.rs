use rutie::{RString, AnyObject, Array, Float};
use cpal::{Stream, StreamConfig};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use ringbuf::RingBuffer;
use std::sync::{Arc, Mutex};

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
        let output = Self::process_audio(&input_str);
        RString::new_utf8(&output).into()
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

    fn process_audio(input: &str) -> String {
        // 여기에 오디오 효과 로직을 구현하세요
        input.to_string()
    }
}