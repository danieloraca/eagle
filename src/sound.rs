use rodio::{Sink, Source};
use std::f32::consts::PI;
use std::time::Duration;
use crate::simple_random::SimpleRng;

pub struct SineWave {
    pub freq: f32,
    pub sample_rate: u32,
    pub duration_samples: u32,
    pub t: u32,
}

impl Iterator for SineWave {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if self.t >= self.duration_samples {
            return None;
        }

        let progress: f32 = self.t as f32 / self.duration_samples as f32;
        let envelope: f32 = 1.0 - progress; // linear fade out

        let sample = (2.0 * PI * self.freq * self.t as f32 / self.sample_rate as f32).sin();
        self.t += 1;

        Some(sample * envelope * 0.3) // slightly louder, but smoothed
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.duration_samples as f32 / self.sample_rate as f32,
        ))
    }
}

struct NoiseBurst {
    duration_samples: u32,
    t: u32,
}

impl Iterator for NoiseBurst {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let mut rng = SimpleRng::new();
        if self.t >= self.duration_samples {
            return None;
        }

        let progress: f32 = self.t as f32 / self.duration_samples as f32;
        let envelope: f32 = 1.0 - progress;

        self.t += 1;
        let sample = rng.random_range_f32(-1.0..1.0); // white noise [-1.0, 1.0]
        Some(sample * envelope * 0.4)
    }
}

impl Source for NoiseBurst {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        44100
    }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.duration_samples as f32 / 44100.0,
        ))
    }
}

struct PitchedTone {
    freq: f32,
    sample_rate: u32,
    duration_samples: u32,
    t: u32,
    waveform: fn(f32) -> f32, // takes phase [0.0..1.0] and returns sample
}

impl Iterator for PitchedTone {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if self.t >= self.duration_samples {
            return None;
        }

        let progress: f32 = self.t as f32 / self.duration_samples as f32;
        let envelope: f32 = 1.0 - progress; // linear fade out

        let phase: f32 = (self.freq * self.t as f32 / self.sample_rate as f32) % 1.0;
        let value: f32 = (self.waveform)(phase) * envelope * 0.3;

        self.t += 1;
        Some(value)
    }
}

impl rodio::Source for PitchedTone {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs_f32(
            self.duration_samples as f32 / self.sample_rate as f32,
        ))
    }
}

fn _sine_wave(phase: f32) -> f32 {
    (2.0 * std::f32::consts::PI * phase).sin()
}

pub fn square_wave(phase: f32) -> f32 {
    if phase < 0.5 { 1.0 } else { -1.0 }
}

fn _triangle_wave(phase: f32) -> f32 {
    4.0 * (phase - 0.5).abs() - 1.0
}

pub fn saw_wave(phase: f32) -> f32 {
    2.0 * phase - 1.0
}

pub fn play_pitched_tone(
    freq: f32,
    duration: f32,
    waveform: fn(f32) -> f32,
    stream_handle: &rodio::OutputStreamHandle,
) {
    if let Ok(sink) = Sink::try_new(stream_handle) {
        let tone: PitchedTone = PitchedTone {
            freq,
            sample_rate: 44100,
            duration_samples: (duration * 44100.0) as u32,
            t: 0,
            waveform,
        };
        sink.append(tone);
        sink.detach();
    }
}

pub fn play_noise_boom(duration: f32, stream_handle: &rodio::OutputStreamHandle) {
    if let Ok(sink) = Sink::try_new(stream_handle) {
        let burst: NoiseBurst = NoiseBurst {
            duration_samples: (duration * 44100.0) as u32,
            t: 0,
        };
        sink.append(burst);
        sink.detach();
    }
}

pub fn _play_combo(
    tones: &[(f32, f32, fn(f32) -> f32)],
    stream_handle: &rodio::OutputStreamHandle,
) {
    if let Ok(sink) = Sink::try_new(stream_handle) {
        for &(freq, dur, wave) in tones {
            let tone: PitchedTone = PitchedTone {
                freq,
                sample_rate: 44100,
                duration_samples: (dur * 44100.0) as u32,
                t: 0,
                waveform: wave,
            };
            sink.append(tone);
        }
        sink.detach();
    }
}
