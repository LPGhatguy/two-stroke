extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate time;

mod gfx_types;
mod vertex;
mod mesh;
mod state;

use std::f32::consts::PI;

use time::precise_time_s;

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::state::{Rasterizer, RasterMethod, FrontFace, CullFace};

use cgmath::prelude::*;
use cgmath::{Quaternion, Vector2, Vector3, Matrix4, Deg, Rad, Euler};

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

fn clamp(value: f32, min: f32, max: f32) -> f32 {
	assert!(min < max);

	if value > max {
		return max;
	}

	if value < min {
		return min;
	}

	value
}

fn handle_event(window: &glutin::Window, state: &mut State, event: glutin::WindowEvent) {
	match event {
		glutin::WindowEvent::Closed | glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) => {
			state.running = false;
		},
		glutin::WindowEvent::KeyboardInput(key_state, _, Some(keycode), _) => {
			match key_state {
				glutin::ElementState::Pressed => {
					state.input.down.insert(keycode);
				},
				glutin::ElementState::Released => {
					state.input.down.remove(&keycode);
				},
			}
		},
		glutin::WindowEvent::MouseMoved(x, y) => {
			match state.input.mouse_position {
				Some(_) => {
					state.input.mouse_move = Some(Vector2::new(x - 200, y - 200));
				},
				None => (),
			}

			state.input.mouse_position = Some(Vector2::new(x, y));
			window.set_cursor_position(200, 200).unwrap();
		},
		_ => ()
	}
}

fn handle_update(state: &mut State) {
	let now = precise_time_s();
	let delta_full = now - state.last_update_time;
	let delta = delta_full as f32;

	state.total_time += delta_full;
	state.last_update_time = now;

	let mut change = Vector3::zero();

	if state.input.down.contains(&glutin::VirtualKeyCode::D) {
		change.x = 1.0;
	} else if state.input.down.contains(&glutin::VirtualKeyCode::A) {
		change.x = -1.0;
	}

	if state.input.down.contains(&glutin::VirtualKeyCode::W) {
		change.z = -1.0;
	} else if state.input.down.contains(&glutin::VirtualKeyCode::S) {
		change.z = 1.0;
	}

	if !change.is_zero() {
		change = change.normalize_to(delta * 3.0);
		change = state.player.camera_orientation.invert().rotate_vector(change);
		state.player.camera_position += change;
	}

	let mut vertical = 0.0f32;

	if state.input.down.contains(&glutin::VirtualKeyCode::E) {
		vertical = 1.0;
	} else if state.input.down.contains(&glutin::VirtualKeyCode::Q) {
		vertical = -1.0;
	}

	if !vertical.is_zero() {
		state.player.camera_position.y += vertical * delta * 3.0;
	}

	match state.input.mouse_move {
		Some(mouse_move) => {
			let pitch = state.player.camera_pitch + (mouse_move.y as f32) * delta * 0.3;
			let pitch = clamp(pitch, -PI / 3.0, PI / 3.0);

			state.player.camera_pitch = pitch;
			state.player.camera_yaw += (mouse_move.x as f32) * delta * 0.3;

			state.player.camera_orientation = Quaternion::from(Euler {
				x: Rad(state.player.camera_pitch),
				y: Rad(state.player.camera_yaw),
				z: Rad(0.0)
			});

			state.input.mouse_move = None;
		},
		None => ()
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

	let shader_set = {
		use gfx::Factory;

		let vs_code = include_bytes!("shader/vertex.glsl");
		let ps_code = include_bytes!("shader/fragment.glsl");

		gfx::ShaderSet::Simple(
			factory.create_shader_vertex(vs_code).expect("Failed to compile vertex shader"),
			factory.create_shader_pixel(ps_code).expect("Failed to compile pixel shader"),
		)
	};

	let pso = factory.create_pipeline_state(
		&shader_set,
		gfx::Primitive::TriangleList,
		Rasterizer {
			front_face: FrontFace::CounterClockwise,
			cull_face: CullFace::Back,
			method: RasterMethod::Line(8),
			offset: None,
			samples: None,
		},
		pipe::new()
	).unwrap();

	let mut state = State::new();
	state.player.camera_position = Vector3::new(0.0, 0.0, 3.0);

	let mut plane = Mesh::plane(&mut factory);
	plane.transform = Matrix4::from_scale(5.0);

	let mut mesh = Mesh::cube(&mut factory);
	mesh.transform = Matrix4::from_translation(Vector3::new(0.0, 2.0, 0.0));

	let projection = cgmath::perspective(Deg(60.0f32), 16.0 / 9.0, 0.05, 100.0);

	let mut data = pipe::Data {
		vbuf: mesh.vertex_buffer.clone(),
		out_color: main_color.clone(),
		out_depth: main_depth.clone(),
		locals: factory.create_constant_buffer(2)
	};

	while state.running {
		handle_update(&mut state);

		events_loop.poll_events(|event| {
			match event {
				glutin::Event::WindowEvent { event, .. } => {
					handle_event(&window, &mut state, event);
				}
			}
		});

		encoder.clear(&main_color, CLEAR_COLOR);
		encoder.clear_depth(&main_depth, 1.0);

		{
			let model_view = Matrix4::from(state.player.camera_orientation) * Matrix4::from_translation(-state.player.camera_position) * mesh.transform;
			let locals = Locals {
				model_view: model_view.into(),
				projection: projection.into()
			};
			encoder.update_constant_buffer(&data.locals, &locals);
			data.vbuf = mesh.vertex_buffer.clone();
			encoder.draw(&mesh.slice, &pso, &data);
		}

		{
			let model_view = Matrix4::from(state.player.camera_orientation) * Matrix4::from_translation(-state.player.camera_position) * plane.transform;
			let locals = Locals {
				model_view: model_view.into(),
				projection: projection.into()
			};
			encoder.update_constant_buffer(&data.locals, &locals);
			data.vbuf = plane.vertex_buffer.clone();
			encoder.draw(&plane.slice, &pso, &data);
		}

		encoder.flush(&mut device);

		window.swap_buffers().unwrap();
		device.cleanup();
	}
}