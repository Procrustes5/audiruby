pub trait Effect {
    fn process(&self, input: &str) -> String;
}

pub struct Distortion;

impl Effect for Distortion {
    fn process(&self, input: &str) -> String {
        format!("Distorted: {}", input)
    }
}

pub struct Compressor {
    threshold: f32,
    ratio: f32,
}

impl Compressor {
    pub fn new(threshold: f32, ratio: f32) -> Self {
        Compressor { threshold, ratio }
    }
}

impl Effect for Compressor {
    fn process(&self, input: &str) -> String {
        format!("Compressed (t:{}, r:{}): {}", self.threshold, self.ratio, input)
    }
}

pub struct Booster {
    gain: f32,
}

impl Booster {
    pub fn new(gain: f32) -> Self {
        Booster { gain }
    }
}

impl Effect for Booster {
    fn process(&self, input: &str) -> String {
        format!("Boosted (g:{}): {}", self.gain, input)
    }
}