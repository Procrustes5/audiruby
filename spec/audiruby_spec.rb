require_relative '../lib/audiruby'

RSpec.describe Audiruby do
  let(:chain) { Audiruby::EffectChain.new }

  describe Audiruby::EffectChain do
    it "processes audio with multiple effects" do
      chain.add_effect(:distortion, amount: 0.5)
      chain.add_effect(:compressor, threshold: 0.3, ratio: 4.0)
      chain.add_effect(:booster, gain: 1.2)

      input = "test_audio"
      output = chain.process(input)

      expect(output).to include("Distorted")
      expect(output).to include("TEST_AUDIO")
    end
  end
end