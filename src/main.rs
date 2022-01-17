use rodio::{OutputStream, Source};
use std::thread;
use std::time::Duration;

mod fork;


struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WaveTableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WaveTableOscillator {
        return WaveTableOscillator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight
            * self.wave_table[truncated_index]
            * next_index_weight
            * self.wave_table[next_index];
    }
}

impl Iterator for WaveTableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample());
    }
}

impl Source for WaveTableOscillator {
    fn channels(&self) -> u16 {
        return 2;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

fn sine(wave_table_size: usize) -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
    }

    return wave_table;
}


fn square(wave_table_size: usize) -> Vec<f32>  {
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for n in 0..wave_table_size {
        if n > wave_table_size / 2 {
            wave_table.push(1.0);
        } else {
            wave_table.push(-1.0);
        }
    }

    return wave_table;
}

fn play_note(wave_table: Vec<f32>, frequency: f32, duration: u64) {

    let mut oscillator = WaveTableOscillator::new(44100, wave_table);
    oscillator.set_frequency(frequency);

    //let oscillator = oscillator.fade_in(Duration::from_millis(duration/4));
    //let oscillator = oscillator.speed(0.1);
    //let oscillator = oscillator.amplify(0.1);
    //let oscillator = oscillator.take_duration(Duration::from_millis(duration/4));
    let oscillator = fork::fadeout(oscillator, Duration::from_millis(duration));

    let (_stream, _stream_handle) = OutputStream::try_default().unwrap();
    let _result = _stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(Duration::from_millis(duration))
}

fn main() {

    let sine = sine(64);

    play_note(sine.clone(), 261.63, 600); //C4
    play_note(sine.clone(), 261.63, 600); //C4
    play_note(sine.clone(), 392.00, 600); //G4
    play_note(sine.clone(), 392.00, 600); //G4
    play_note(sine.clone(), 440.00, 600); //A4
    play_note(sine.clone(), 440.00, 600); //A4
    play_note(sine.clone(), 392.00, 1200); //C4

    play_note(sine.clone(), 349.23, 600); //F4
    play_note(sine.clone(), 349.23, 600); //F4
    play_note(sine.clone(), 329.63, 600); //E4
    play_note(sine.clone(), 329.63, 600); //E4
    play_note(sine.clone(), 293.66, 600); //D4
    play_note(sine.clone(), 293.66, 600); //D4
    play_note(sine.clone(), 261.63, 1200); //C4

    play_note(sine.clone(), 392.00, 600); //G4
    play_note(sine.clone(), 392.00, 600); //G4
    play_note(sine.clone(), 349.23, 600); //F4
    play_note(sine.clone(), 349.23, 600); //F4
    play_note(sine.clone(), 329.63, 600); //E4
    play_note(sine.clone(), 329.63, 600); //E4
    play_note(sine.clone(), 293.66, 1200); //D

    play_note(sine.clone(), 392.00, 600); //G4
    play_note(sine.clone(), 392.00, 600); //G4
    play_note(sine.clone(), 349.23, 600); //F4
    play_note(sine.clone(), 349.23, 600); //F4
    play_note(sine.clone(), 329.63, 600); //E4
    play_note(sine.clone(), 329.63, 600); //E4
    play_note(sine.clone(), 293.66, 1200); //D4

    play_note(sine.clone(), 261.63, 600); //C4
    play_note(sine.clone(), 261.63, 600); //C4
    play_note(sine.clone(), 392.00, 600); //G4
    play_note(sine.clone(), 392.00, 600); //G4
    play_note(sine.clone(), 440.00, 600); //A4
    play_note(sine.clone(), 440.00, 600); //A4
    play_note(sine.clone(), 392.00, 1200); //G4

    thread::spawn(|| {
        let square = square(64);
        play_note(square.clone(), 87.31, 1200); //F2
        play_note(square.clone(), 82.41, 1200); //E2
        play_note(square.clone(), 73.42, 1200); //D2
        play_note(square.clone(), 65.41, 1200); //C2
    });

    play_note(sine.clone(), 349.23, 600); //F4
    play_note(sine.clone(), 349.23, 600); //F4
    play_note(sine.clone(), 329.63, 600); //E4
    play_note(sine.clone(), 329.63, 600); //E4
    play_note(sine.clone(), 293.66, 600); //D4
    play_note(sine.clone(), 293.66, 600); //D4
    play_note(sine.clone(), 261.63, 1200); //C4  
}
