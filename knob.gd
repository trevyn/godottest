extends Node2D

#signal turnedKnob

var following := false
const MAX_DIST := 7000

func _physics_process(delta: float) -> void:
	var mouseDist := get_global_mouse_position().distance_squared_to(global_position)
	if mouseDist < MAX_DIST and Input.is_action_just_pressed("click"):
		following = true
	if Input.is_action_just_released("click"):
		following = false
		
	if following:
		var ang := get_global_mouse_position().angle_to_point(global_position) + PI/2
		rotation = ang
		var freq = 440+(ang*40)
		$"../Generator".pulse_hz = freq
		$Label.text = str(int(freq))
#		var d :Vector2= ($knob/knobPoint.position.rotated(.rotation))
#		var a = $middlePoint.position.angle_to(d)
#		var finalAng :float= range_lerp( a, -3.14, 3.14, 0, 100 )
#		print(finalAng)
#		var fang :float= lerp_angle( $knob.rotation, ang, 0.3  )
#		$knob.rotation = clamp(fang, -2,2)
#		emit_signal("turnedKnob", finalAng)
