use std::collections::HashSet;

use cgmath::{Zero, One, Vector2, Vector3, Quaternion};
use glfw::Key;
use time::precise_time_s;

use mesh::Mesh;

pub struct InputState {
	pub down: HashSet<Key>,
	pub last_mouse_position: Vector2<f64>,
	pub mouse_position: Vector2<f64>
}

impl InputState {
	pub fn new() -> InputState {
		InputState {
			down: HashSet::new(),
			last_mouse_position: Vector2::new(0.0, 0.0),
			mouse_position: Vector2::new(0.0, 0.0),
		}
	}

	pub fn get_mouse_delta(&self) -> Vector2<f64> {
		self.mouse_position - self.last_mouse_position
	}
}

pub struct PlayerState {
	pub camera_pitch: f32,
	pub camera_yaw: f32,
	pub camera_position: Vector3<f32>,
	pub camera_orientation: Quaternion<f32>,
}

impl PlayerState {
	pub fn new() -> Self {
		PlayerState {
			camera_pitch: 0.0,
			camera_yaw: 0.0,
			camera_position: Vector3::zero(),
			camera_orientation: Quaternion::one(),
		}
	}
}

pub struct State {
	pub start_time: f64,
	pub last_update_time: f64,
	pub total_time: f64,
	pub running: bool,
	pub meshes: Vec<Mesh>,
	pub input: InputState,
	pub player: PlayerState,
}

impl State {
	pub fn new() -> Self {
		State {
			start_time: precise_time_s(),
			last_update_time: precise_time_s(),
			total_time: 0.0,

			running: true,
			meshes: vec![],
			input: InputState::new(),
			player: PlayerState::new(),
		}
	}
}