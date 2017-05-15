use mesh::Mesh;
use input_state::InputState;

pub struct State {
	pub running: bool,
	pub meshes: Vec<Mesh>,
	pub input: InputState
}

impl State {
	pub fn new() -> State {
		State {
			running: true,
			meshes: vec![],
			input: InputState::new()
		}
	}
}