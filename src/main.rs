extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glfw;
extern crate glfw;
extern crate time;
extern crate rusttype;

mod gfx_types;
mod vertex;
mod mesh;
mod state;

use std::f32::consts::PI;
use std::cmp;

use time::precise_time_s;

use glfw::{Action, Context, Key};

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx::state::{Rasterizer, RasterMethod, FrontFace, CullFace};
use gfx::format::Rgba8;

use cgmath::prelude::*;
use cgmath::{Quaternion, Vector2, Vector3, Matrix4, Deg, Rad, Euler};

use rusttype::{FontCollection, PositionedGlyph};

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

gfx_defines! {
	vertex TextVertex {
	    pos: [f32; 2] = "a_Pos",
	    uv: [f32; 2] = "a_Uv",
	}

	pipeline text_pipe {
		vbuf: gfx::VertexBuffer<TextVertex> = (),
		image: gfx::TextureSampler<[f32; 4]> = "t_texture",
		out: gfx::RenderTarget<ColorFormat> = "Target0",
	}
}

impl TextVertex {
	pub fn new(pos: [f32; 2], uv: [f32; 2]) -> TextVertex {
		TextVertex {
			pos,
			uv
		}
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

fn handle_event(window: &mut glfw::Window, state: &mut State, event: glfw::WindowEvent) {
	match event {
		glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
			window.set_should_close(true);
		},
		glfw::WindowEvent::Key(keycode, _, action, _) => {
			match action {
				Action::Press => {
					state.input.down.insert(keycode);
				},
				Action::Release => {
					state.input.down.remove(&keycode);
				},
				_ => ()
			}
		},
		glfw::WindowEvent::CursorPos(x, y) => {
			state.input.mouse_position = Vector2::new(x, y);
		},
		_ => ()
	}
}

fn handle_update(state: &mut State) {
	let now = precise_time_s();
	let delta_full = now - state.last_update_time;
	let delta = delta_full as f32;

	let mouse_delta = state.input.get_mouse_delta();

	state.input.last_mouse_position = state.input.mouse_position;

	state.total_time += delta_full;
	state.last_update_time = now;

	let mut change = Vector3::zero();

	if state.input.down.contains(&glfw::Key::D) {
		change.x = 1.0;
	} else if state.input.down.contains(&glfw::Key::A) {
		change.x = -1.0;
	}

	if state.input.down.contains(&glfw::Key::W) {
		change.z = -1.0;
	} else if state.input.down.contains(&glfw::Key::S) {
		change.z = 1.0;
	}

	if !change.is_zero() {
		change = change.normalize_to(delta * 3.0);
		change = state.player.camera_orientation.invert().rotate_vector(change);
		state.player.camera_position += change;
	}

	let mut vertical = 0.0f32;

	if state.input.down.contains(&glfw::Key::E) {
		vertical = 1.0;
	} else if state.input.down.contains(&glfw::Key::Q) {
		vertical = -1.0;
	}

	if !vertical.is_zero() {
		state.player.camera_position.y += vertical * delta * 3.0;
	}

	if !mouse_delta.is_zero() {
		let turn_rate = 0.3;
		let pitch = state.player.camera_pitch + (mouse_delta.y as f32) * delta * turn_rate;
		let pitch = clamp(pitch, -PI / 3.0, PI / 3.0);

		state.player.camera_pitch = pitch;
		state.player.camera_yaw += (mouse_delta.x as f32) * delta * turn_rate;

		state.player.camera_orientation = Quaternion::from(Euler {
			x: Rad(state.player.camera_pitch),
			y: Rad(state.player.camera_yaw),
			z: Rad(0.0)
		});
	}
}

fn main() {
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
		.ok()
		.expect("Failed to initialize GLFW");

	glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
	glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(false));
	glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

	let (mut window, events) = glfw.create_window(1024, 768, "Window example", glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window.");

	window.set_all_polling(true);
	window.set_cursor_mode(glfw::CursorMode::Disabled);
	window.set_cursor_pos(0.0, 0.0);

	window.make_current();

	glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
	glfw.set_error_callback(glfw::FAIL_ON_ERRORS);

	let (mut device, mut factory, main_color, main_depth) = gfx_window_glfw::init(&mut window);

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

	let text_shader_set = {
		use gfx::Factory;

		let vs_code = include_bytes!("shader/text-vertex.glsl");
		let ps_code = include_bytes!("shader/text-fragment.glsl");

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
			method: RasterMethod::Fill,
			offset: None,
			samples: None,
		},
		pipe::new()
	).unwrap();

	let pso_lines = factory.create_pipeline_state(
		&shader_set,
		gfx::Primitive::TriangleList,
		Rasterizer {
			front_face: FrontFace::CounterClockwise,
			cull_face: CullFace::Nothing,
			method: RasterMethod::Line(2),
			offset: None,
			samples: None,
		},
		pipe::new()
	).unwrap();

	let pso_text = factory.create_pipeline_state(
		&text_shader_set,
		gfx::Primitive::TriangleList,
		Rasterizer {
			front_face: FrontFace::CounterClockwise,
			cull_face: CullFace::Nothing,
			method: RasterMethod::Fill,
			offset: None,
			samples: None,
		},
		text_pipe::new()
	).unwrap();

	let mut state = State::new();
	state.player.camera_position = Vector3::new(0.0, 0.0, 0.0);

	let mut plane = Mesh::plane(&mut factory, 16);
	plane.transform = Matrix4::from_translation(Vector3::new(0.0, -1.0, 0.0));

	let mut mesh = Mesh::cube(&mut factory);
	mesh.transform = Matrix4::from_translation(Vector3::new(0.0, 0.0, 2.0));

	let projection = cgmath::perspective(Deg(60.0f32), 16.0 / 9.0, 0.05, 100.0);

	let mut data = pipe::Data {
		vbuf: mesh.vertex_buffer.clone(),
		out_color: main_color.clone(),
		out_depth: main_depth.clone(),
		locals: factory.create_constant_buffer(2)
	};

	let font_data = include_bytes!("font/Roboto-Light.ttf");
	let collection = FontCollection::from_bytes(font_data as &[u8]);
	let font = collection.into_font().unwrap();

	let text_height: f32 = 12.4;
	let pixel_height = text_height.ceil() as usize;

	let scale = rusttype::Scale {
		x: text_height * 2.0,
		y: text_height,
	};

	let v_metrics = font.v_metrics(scale);
	let offset = rusttype::point(0.0, v_metrics.ascent);

	let glyphs: Vec<PositionedGlyph> = font.layout("Hello, world!", scale, offset).collect();

	let mut min_x = 0;
	let mut min_y = 0;
	let mut max_x = 0;
	let mut max_y = 0;

	for glyph in &glyphs {
		if let Some(bb) = glyph.pixel_bounding_box() {
			min_x = cmp::min(min_x, bb.min.x);
			min_y = cmp::min(min_y, bb.min.y);
			max_x = cmp::max(max_x, bb.max.x);
			max_y = cmp::max(max_y, bb.max.y);

			println!("{:?}", bb);
		}
	}

	let width = (max_x - min_x) as usize;
	let height = (max_y - min_y) as usize;

	let mut texture = Vec::<u8>::with_capacity(width * height * 4);

	for x in 0..(width * height) {
		texture.push(255);
		texture.push(255);
		texture.push(255);
		texture.push(0);
	}

	println!("({}, {}) to ({}, {})", min_x, min_y, max_x, max_y);

	for glyph in &glyphs {
		glyph.draw(|x, y, v| {
			if let Some(bb) = glyph.pixel_bounding_box() {
				let x = (x as i32 + bb.min.x) as usize;
				let y = (y as i32 + bb.min.y) as usize;

				texture[y * width * 4 + x * 4 + 3] = (v * 255.0) as u8;
			}
		});
	}

	println!("({}, {})", width, height);

	let texture_view = {
		use gfx::Factory;

		let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
		let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, &[texture.as_slice()]).unwrap();

		view
	};
	let sampler = factory.create_sampler_linear();

	let text_vertices = [
		TextVertex::new([-1.0, -1.0], [0.0, 1.0]),
		TextVertex::new([ 1.0, -1.0], [1.0, 1.0]),
		TextVertex::new([ 1.0,  1.0], [1.0, 0.0]),

		TextVertex::new([-1.0, -1.0], [0.0, 1.0]),
		TextVertex::new([ 1.0,  1.0], [1.0, 0.0]),
		TextVertex::new([-1.0,  1.0], [0.0, 0.0]),
	];
	let (text_vbuf, text_slice) = factory.create_vertex_buffer_with_slice(&text_vertices, ());

	let text_data = text_pipe::Data {
		vbuf: text_vbuf,
		image: (texture_view, sampler.clone()),
		out: main_color.clone(),
	};

	while !window.should_close() {
		handle_update(&mut state);

		glfw.poll_events();

		for (_, event) in glfw::flush_messages(&events) {
			handle_event(&mut window, &mut state, event);
		}

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
			encoder.draw(&plane.slice, &pso_lines, &data);
		}

		encoder.flush(&mut device);

		window.swap_buffers();
		device.cleanup();
	}
}