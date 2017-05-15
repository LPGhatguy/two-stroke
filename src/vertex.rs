gfx_vertex_struct! {
	Vertex {
		position: [f32; 3] = "a_Pos",
		color: [f32; 3] = "a_Color",
	}
}

impl Vertex {
	pub fn new(x: f32, y: f32, z: f32) -> Vertex {
		Vertex { position: [x, y, z], color: [0.0, 0.2, 0.8] }
	}

	pub fn new_color(x: f32, y: f32, z: f32, g: f32) -> Vertex {
		Vertex { position: [x, y, z], color: [g, g, g] }
	}
}