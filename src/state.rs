use std::collections::HashSet;

use cgmath::{Zero, One, Vector3, Quaternion};
use glutin::VirtualKeyCode;
use time::precise_time_s;

use mesh::Mesh;

pub struct InputState {
	pub down: HashSet<VirtualKeyCode>,
}

impl InputState {
	pub fn new() -> InputState {
		InputState {
			down: HashSet::new()
		}
	}
}

pub struct PlayerState {
	pub camera_position: Vector3<f32>,
	pub orientation: Quaternion<f32>,
}

impl PlayerState {
	pub fn new() -> Self {
		PlayerState {
			camera_position: Vector3::zero(),
			orientation: Quaternion::one(),
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