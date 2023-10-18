// cargo build --release --target aarch64-apple-darwin

#![allow(unused)]
use freeverb::Freeverb;
use godot::engine::*;
use godot::prelude::*;
use std::f64;

fn main() {
    struct HelloWorld;

    #[gdextension]
    unsafe impl ExtensionLibrary for HelloWorld {}
}

#[derive(GodotClass)]
#[class(base=Node)]
struct Generator {
    sample_hz: f32,
    #[export]
    pulse_hz: f32,
    phase: f32,
    playback: Option<Gd<AudioStreamGeneratorPlayback>>,
    freeverb: Freeverb,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl NodeVirtual for Generator {
    fn init(base: Base<Node>) -> Self {
        Self {
            sample_hz: 44100.0,
            pulse_hz: 440.0,
            phase: 0.0,
            playback: None,
            freeverb: Freeverb::new(44100),
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        self.fill_buffer()
    }

    fn ready(&mut self) {
        let mut audio_stream_player = self
            .base
            .get_node_as::<AudioStreamPlayer>("AudioStreamPlayer");

        let mut audio_stream_generator = audio_stream_player
            .get_stream()
            .unwrap()
            .cast::<AudioStreamGenerator>();

        audio_stream_generator.set_mix_rate(self.sample_hz);
        audio_stream_generator.set_buffer_length(0.024);
        audio_stream_player.play();
        self.playback = Some(audio_stream_player.get_stream_playback().unwrap().cast());
        self.fill_buffer();
    }
}

#[godot_api]
impl Generator {
    fn fill_buffer(&mut self) {
        let increment = self.pulse_hz / self.sample_hz;

        let playback = self.playback.as_mut().unwrap();

        let to_fill = playback.get_frames_available();
        for _ in 0..to_fill {
            let sample = (self.phase as f64 * f64::consts::TAU).sin();
            self.phase = (self.phase + increment) % 1.0;
            let verbed_sample = self.freeverb.tick((sample, sample));
            let v = Vector2::new(verbed_sample.0 as f32, verbed_sample.1 as f32);
            playback.push_frame(v);
        }

        self.base
            .get_node_as::<Label>("Label")
            .set_text(format!("Skips = {}", playback.get_skips()).into());
    }
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    #[base]
    base: Base<Sprite2D>,
}

#[godot_api]
impl Sprite2DVirtual for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello world from Rust!");

        Self {
            speed: 400.0,
            angular_speed: f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.base.rotate((self.angular_speed * delta) as f32);
    }
}
