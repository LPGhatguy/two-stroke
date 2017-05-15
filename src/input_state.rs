use cgmath::{Zero, Vector2};

pub struct InputState {
	pub movement: Vector2<f32>,
}

impl InputState {
	pub fn new() -> InputState {
		InputState {
			movement: Vector2::<f32>::zero()
		}
	}
}