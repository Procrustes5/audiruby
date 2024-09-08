#[macro_use]
extern crate rutie;

use rutie::{Class, Object, RString, AnyObject, VM};

mod audio_processor;

class!(AudioProcessor);

methods!(
    AudioProcessor,
    _rtself,

    fn ruby_process(input: RString) -> AnyObject {
        println!("Rust: process called with input: {:?}", input);
        match input.map_err(|e| VM::raise_ex(e)) {
            Ok(s) => audio_processor::AudioProcessor::process(s),
            Err(_) => RString::new_utf8("Error processing input").into()
        }
    }

    fn ruby_start_audio_capture() -> AnyObject {
        println!("Rust: start_audio_capture called");
        audio_processor::AudioProcessor::start_audio_capture()
    }

    fn ruby_get_audio_data() -> AnyObject {
        println!("Rust: get_audio_data called");
        audio_processor::AudioProcessor::get_audio_data()
    }

    fn ruby_analyze_audio() -> AnyObject {
        println!("Rust: analyze_audio called");
        audio_processor::AudioProcessor::analyze_audio()
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_audiruby() {
    let mut class = Class::new("AudioProcessor", None);
    class.define(|klass| {
        klass.def_self("_rust_start_audio_capture", ruby_start_audio_capture);
        klass.def_self("_rust_get_audio_data", ruby_get_audio_data);
        klass.def_self("_rust_analyze_audio", ruby_analyze_audio);
        klass.def_self("process", ruby_process);
    });
}