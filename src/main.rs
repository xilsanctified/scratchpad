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
    frequency: Option<i32>,
}
impl Oscillator {
    fn new_osc() -> Oscillator {
        Oscillator {
            wave_type: Wave::Saw,
            frequency: None,
        }
    }
    fn update(&mut self, freq: Option<i32>) {
        self.frequency = freq;
    }
    fn describe(&self) -> String {
        let mut outstr = String::from(&self.wave_type.describe());

        outstr.push_str("\nFrequency set to: ");

        let freq: String = match &self.frequency {
            None => String::from("Not Initialized!"),
            Some(s) => s.to_string(),
        };

        outstr.push_str(&freq);
        outstr.push_str(".");
        outstr
    }
}
struct Synthesizer {
    osc1: Oscillator,
    osc2: Oscillator,
    voice: Voice,
}
impl Synthesizer {
    fn new() -> Synthesizer {
        let new_synth = Synthesizer {
            osc1: Oscillator::new_osc(),
            osc2: Oscillator::new_osc(),
            voice: Voice::Sixteen,
        };
        new_synth
    }
    fn describe(&self) {
        println!("Oscillator 1: {}", self.osc1.describe());
        println!("Wave Type: {}", self.osc1.wave_type.describe());
        println!("Oscillator 2: {}", self.osc2.describe());
        println!("Wave Type: {}", self.osc2.wave_type.describe());
        println!("Voices: {}", self.voice.describe());
    }
    fn initialize(&mut self, freq: Option<i32>) {
        self.osc1.update(freq);
        self.osc2.update(freq);
    }
}

fn main() {
    let mut synth = Synthesizer::new();

    println!("#####################");

    synth.describe();

    synth.initialize(Some(300));

    println!("#####################");

    synth.describe();
}
