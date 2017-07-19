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
		let area = (size - 1) * (size - 1);

		let mut vertices = Vec::<Vertex>::with_capacity((size + 1) * (size + 1));

		// Number of triangles is 2 * area, 3 vertices per triangle
		let mut indices = Vec::<u16>::with_capacity(3 * 2 * area);

		let width = size as u16;

		for x in 0..size {
			for z in 0..size {
				vertices.push(Vertex::new(
					x as f32,
					0.0,
					z as f32
				))
			}
		}

		for x in 0..(width - 1) {
			for y in 0..(width - 1) {
				let n = x + y * width;

				indices.push(n);
				indices.push(n + width + 1);
				indices.push(n + 1);

				indices.push(n);
				indices.push(n + width);
				indices.push(n + width + 1);
			}
		}

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