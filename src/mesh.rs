use vertex::Vertex;

use gfx;
use gfx::IntoIndexBuffer;
use gfx::traits::FactoryExt;
use cgmath::{One, Matrix4};

use gfx_types::{Resources, Factory};

pub struct Mesh {
	pub vertex_buffer: gfx::handle::Buffer<Resources, Vertex>,
	pub slice: gfx::Slice<Resources>,
	pub transform: Matrix4<f32>,
}

impl Mesh {
	pub fn new(factory: &mut Factory, vertices: &Vec<Vertex>, elements: &Vec<u16>) -> Mesh {
		let index_buffer = elements.into_index_buffer(factory);
		let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(vertices.as_slice(), index_buffer);

		Mesh {
			vertex_buffer: vertex_buffer,
			slice: slice,
			transform: Matrix4::one(),
		}
	}

	pub fn plane(factory: &mut Factory, size: usize) -> Mesh {
		let vertices = vec![
			Vertex::new(-0.5, 0.0, -0.5),
			Vertex::new(-0.5, 0.0, 0.5),
			Vertex::new(0.5, 0.0, 0.5),
			Vertex::new(0.5, 0.0, -0.5),
		];
		let indices: Vec<u16> = vec![
			0, 1, 3,
			3, 1, 2,
		];

		Mesh::new(factory, &vertices, &indices)
	}

	pub fn cube(factory: &mut Factory) -> Mesh {
		let vertices = vec![
			// Front face
			Vertex::new_color(0.5, -0.5, 0.5, 0.25),
			Vertex::new(-0.5, -0.5, 0.5),
			Vertex::new(-0.5, 0.5, 0.5),
			Vertex::new(0.5, 0.5, 0.5),

			// Back face
			Vertex::new(-0.5, 0.5, -0.5),
			Vertex::new(0.5, 0.5, -0.5),
			Vertex::new_color(0.5, -0.5, -0.5, 1.0),
			Vertex::new(-0.5, -0.5, -0.5),
		];

		let indices: Vec<u16> = vec![
			// Front face
			0, 2, 1,
			0, 3, 2,

			// Back face
			6, 4, 5,
			6, 7, 4,

			// // Right face
			5, 0, 6,
			5, 3, 0,

			// Left face
			1, 4, 7,
			1, 2, 4,

			// Top face
			4, 2, 3,
			4, 3, 5,

			// Bottom face
			7, 0, 1,
			7, 6, 0,
		];

		Mesh::new(factory, &vertices, &indices)
	}
}