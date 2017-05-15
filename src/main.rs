extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;

mod gfx_types;
mod vertex;
mod mesh;
mod state;
mod input_state;

use gfx::traits::FactoryExt;
use gfx::Device;

use cgmath::{Vector3, Matrix4, Quaternion, Deg};
use cgmath::One;

use state::State;
use mesh::Mesh;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
	constant Locals {
		model_view: [[f32; 4]; 4] = "u_ModelView",
		projection: [[f32; 4]; 4] = "u_Projection",
	}

	pipeline pipe {
		vbuf: gfx::VertexBuffer<::vertex::Vertex> = (),
		out_color: gfx::RenderTarget<ColorFormat> = "Target0",
		out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
		locals: gfx::ConstantBuffer<Locals> = "Locals",
	}
}

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

fn handle_event(state: &mut State, event: glutin::WindowEvent) {
	match event {
		glutin::WindowEvent::Closed | glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) => {
			state.running = false;
		},
		_ => ()
	}
}

fn main() {
	let events_loop = glutin::EventsLoop::new();
	let builder = glutin::WindowBuilder::new()
		.with_title("Triangle example".to_string())
		.with_dimensions(1280, 720)
		.with_vsync();

	let (window, mut device, mut factory, main_color, main_depth) =
		gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

	let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

	let pso = factory.create_pipeline_simple(
		include_bytes!("shader/vertex.glsl"),
		include_bytes!("shader/fragment.glsl"),
		pipe::new()
	).unwrap();

	let mut state = State::new();
	let mesh = Mesh::cube(&mut factory);

	let mut step = 0.0f32;

	let model_view = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));

	let projection = cgmath::perspective(Deg(60.0f32), 16.0 / 9.0, 0.05, 100.0);

	let data = pipe::Data {
		vbuf: mesh.vertex_buffer.clone(),
		out_color: main_color.clone(),
		out_depth: main_depth.clone(),
		locals: factory.create_constant_buffer(2)
	};

	while state.running {
		step += 0.01;

		let model_view = Matrix4::from_translation(Vector3::new(step.cos(), step.sin(), -2.0));

		events_loop.poll_events(|event| {
			match event {
				glutin::Event::WindowEvent { event, .. } => {
					handle_event(&mut state, event);
				}
			}
		});

		{
			let locals = Locals {
				model_view: model_view.into(),
				projection: projection.into()
			};
			encoder.update_constant_buffer(&data.locals, &locals);
		}

		encoder.clear(&main_color, CLEAR_COLOR);
		encoder.clear_depth(&main_depth, 1.0);

		encoder.draw(&mesh.slice, &pso, &data);

		encoder.flush(&mut device);

		window.swap_buffers().unwrap();
		device.cleanup();
	}
}