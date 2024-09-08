require 'rutie'
require 'tk'

module Audiruby
  puts "Attempting to load Rust library..."
  begin
    Rutie.new(:audiruby).init 'Init_audiruby', File.expand_path('../release', __dir__)
    puts "Rust library loaded successfully"
  rescue => e
    puts "Failed to load Rust library: #{e.message}"
    puts e.backtrace
    exit 1
  end

  class AudioProcessor
    class << self
      def start_audio_capture
        puts "Ruby: Calling start_audio_capture"
        begin
          result = call_rust_start_audio_capture
          puts "Ruby: start_audio_capture result: #{result.inspect}"
          result
        rescue => e
          puts "Error in start_audio_capture: #{e.message}"
          puts e.backtrace
          raise
        end
      end

      def analyze_audio
        puts "Ruby: Calling analyze_audio"
        result = _rust_analyze_audio
        puts "Ruby: analyze_audio result: #{result.inspect}"
        result
      end

      def process(input)
        puts "Ruby: Calling process"
        begin
          result = _rust_process(input)
          puts "Ruby: process result: #{result.inspect}"
          result
        rescue => e
          puts "Ruby: Error in process: #{e.message}"
          nil
        end
      end

      def _rust_start_audio_capture
        puts "Ruby: Calling Rust method _rust_start_audio_capture"
        AudioProcessor._rust_start_audio_capture
      rescue => e
        puts "Error calling Rust method: #{e.message}"
        puts e.backtrace
        raise
      end

      private

      def call_rust_start_audio_capture
        puts "Ruby: Calling Rust method _rust_start_audio_capture"
        Object.const_get(:AudioProcessor)._rust_start_audio_capture
      rescue => e
        puts "Error calling Rust method: #{e.message}"
        puts e.backtrace
        raise
      end
    end
  end

  class GUI
    def initialize
      @root = TkRoot.new { title "Audio Analyzer" }
      @frequency_label = TkLabel.new(@root) { text "Frequency: " }
      @chord_label = TkLabel.new(@root) { text "Chord: " }
      @start_button = TkButton.new(@root) { text "Start Capture" }
      @stop_button = TkButton.new(@root) { text "Stop Capture"; state "disabled" }

      @frequency_label.pack
      @chord_label.pack
      @start_button.pack
      @stop_button.pack

      @start_button.command { start_capture }
      @stop_button.command { stop_capture }

      @update_timer = nil
    end

    def start_capture
      AudioProcessor.start_audio_capture
      @start_button.state = "disabled"
      @stop_button.state = "normal"
      @update_timer = TkAfter.new(100, -1, proc { update_analysis })
    end

    def stop_capture
      @update_timer.cancel if @update_timer
      @start_button.state = "normal"
      @stop_button.state = "disabled"
      @frequency_label.text = "Frequency: "
      @chord_label.text = "Chord: "
    end

    def update_analysis
      result = AudioProcessor.analyze_audio
      @frequency_label.text = "Frequency: #{result['frequency'].round(2)} Hz"
      @chord_label.text = "Chord: #{result['chord']}"
    end

    def run
      Tk.mainloop
    end
  end
end

# Run the application
gui = Audiruby::GUI.new
gui.run