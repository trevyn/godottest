extends Node


var sample_hz = 22050.0 # Keep the number of samples to mix low, GDScript is not super fast.
var pulse_hz = 440.0
var phase = 0.0

var playback: AudioStreamPlayback = null # Actual playback stream, assigned in _ready().

func do_thing():
	pulse_hz = 392 if pulse_hz==440 else 440

func _process(_delta):
	_fill_buffer()

func _fill_buffer():
	var increment = pulse_hz / sample_hz

	var to_fill = playback.get_frames_available()
	while to_fill > 0:
		playback.push_frame(Vector2.ONE * sin(phase * TAU)) # Audio frames are stereo.
		phase = fmod(phase + increment, 1.0)
		to_fill -= 1

func _ready():
	$AudioStreamPlayer.stream.mix_rate = sample_hz
	$AudioStreamPlayer.stream.buffer_length = 0.015
	$AudioStreamPlayer.play()
	playback = $AudioStreamPlayer.get_stream_playback()
	_fill_buffer()

