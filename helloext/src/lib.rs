// cargo build --release --target aarch64-apple-darwin

#![allow(unused)]
use godot::engine::audio_stream_player;
use godot::engine::AudioStreamGenerator;
use godot::engine::AudioStreamGeneratorPlayback;
use godot::engine::AudioStreamPlayback;
use godot::engine::AudioStreamPlayer;
use godot::engine::Label;
use godot::engine::Sprite2D;
use godot::engine::Sprite2DVirtual;
use godot::prelude::*;

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
        for i in 0..to_fill {
            let v = Vector2::ONE * (self.phase * std::f32::consts::TAU).sin();
            self.phase = (self.phase + increment) % 1.0;
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
    sprite: Base<Sprite2D>,
}

#[godot_api]
impl Sprite2DVirtual for Player {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Hello world from Rust!");

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            sprite,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        self.sprite.rotate((self.angular_speed * delta) as f32);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }
}
