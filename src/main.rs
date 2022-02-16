enum Wave {
    Square,
    Sine,
    Saw,
    Triangle,
}
impl Wave {
    fn describe(&self) -> String {
        match &self {
            Wave::Saw => String::from("Set to Saw."),
            Wave::Sine => String::from("Set to Sine."),
            Wave::Square => String::from("Set to Square."),
            Wave::Triangle => String::from("Set to Triangle."),
        }
    }
}
enum Voice {
    Sixteen,
    Eight,
}
impl Voice {
    fn describe(&self) -> String {
        match &self {
            Voice::Sixteen => String::from("16 voice synthesizer."),
            Voice::Eight => String::from("8 voice synthesizer."),
        }
    }
}
struct Oscillator {
    wave_type: Wave,
    frequency: u32,
}
impl Oscillator {
    fn new_osc() -> Oscillator {
        Oscillator {
            wave_type: Wave::Saw,
            frequency: 0,
        }
    }

    fn describe(&self) -> String {
        let mut outstr = String::from(&self.wave_type.describe());
        let freq = &self.frequency.to_string();
        outstr.push_str("\nFrequency set to: ");
        outstr.push_str(freq);
        outstr.push_str(".");
        outstr
    }
}
struct Synthesizer {
    osc1: Oscillator,
    osc2: Oscillator,
    wave: Wave,
    voice: Voice,
}
impl Synthesizer {
    fn describe(&self) {
        println!("Oscillator 1: {}", self.osc1.describe());
        println!("Oscillator 2: {}", self.osc2.describe());
        println!("Wave Type: {}", self.wave.describe());
        println!("Voices: {}", self.voice.describe());
    }
}

fn main() {
    let synth = Synthesizer {
        wave: Wave::Saw,
        osc1: Oscillator::new_osc(),
        osc2: Oscillator::new_osc(),
        voice: Voice::Sixteen,
    };

    synth.describe();
}
