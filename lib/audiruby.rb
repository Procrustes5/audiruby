require 'rutie'

module Audiruby
  Rutie.new(:audiruby).init 'Init_audiruby', File.expand_path('../release', __dir__)

  class AudioProcessor
    class << self
      def start_audio_capture
        puts "Ruby: Calling start_audio_capture"
        result = _rust_start_audio_capture
        puts "Ruby: start_audio_capture result: #{result.inspect}"
        result
      end

      private

      def _rust_start_audio_capture
        raise NotImplementedError, "Rust method not properly linked"
      end
    end
  end
end
