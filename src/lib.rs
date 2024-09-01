#[macro_use]
extern crate rutie;

use rutie::{Class, Object, RString, AnyObject, VM};

mod audio_processor;
mod effects;

class!(RubyAudioProcessor);

methods!(
    RubyAudioProcessor,
    _rtself,

    fn ruby_process(input: RString) -> AnyObject {
        println!("Rust: process called with input: {:?}", input);
        let input_str = input.map_err(|e| VM::raise_ex(e)).unwrap();
        let result = audio_processor::AudioProcessor::process(input_str);
        println!("Rust: process result: {:?}", result);
        result
    }

    fn ruby_start_audio_capture() -> AnyObject {
        println!("Rust: start_audio_capture called");
        // 실제 오디오 캡처 동작 수행
        RString::new_utf8("Audio capture started").into()
    }

    fn ruby_get_audio_data() -> AnyObject {
        println!("Rust: get_audio_data called");
        let result = audio_processor::AudioProcessor::get_audio_data();
        println!("Rust: get_audio_data result: {:?}", result);
        result
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_audiruby() {
    Class::new("AudioProcessor", None).define(|klass| {
        klass.def_self("_rust_start_audio_capture", ruby_start_audio_capture);
    });
}
