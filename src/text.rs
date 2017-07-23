use std::cmp;

use rusttype;
use rusttype::{FontCollection, PositionedGlyph};

use texture::Texture;

pub struct Font<'a> {
	font: rusttype::Font<'a>,
}

impl<'a> Font<'a> {
	pub fn from_bytes(font_data: &[u8]) -> Option<Font> {
		let collection = FontCollection::from_bytes(font_data);

		if let Some(font) = collection.into_font() {
			Some(Font {
				font,
			})
		} else {
			None
		}
	}

	pub fn layout(&self, text: &str, height: f32) -> Vec<PositionedGlyph> {
		let scale = rusttype::Scale {
			x: height,
			y: height,
		};

		let v_metrics = self.font.v_metrics(scale);
		let offset = rusttype::point(0.0, v_metrics.ascent);

		self.font.layout(text, scale, offset).collect()
	}

	pub fn render(&self, text: &str, height: f32) -> Texture {
		let glyphs = self.layout(text, height);
		let (width, height) = get_texture_size_from_glyphs(&glyphs);

		let mut texture_data = Vec::<u8>::with_capacity(width * height * 4);

		for _ in 0..(width * height) {
			texture_data.push(0);
			texture_data.push(0);
			texture_data.push(0);
			texture_data.push(0);
		}

		for glyph in glyphs.iter() {
			if let Some(bb) = glyph.pixel_bounding_box() {
				glyph.draw(|x, y, v| {
					let x = (x as i32 + bb.min.x) as usize;
					let y = (y as i32 + bb.min.y) as usize;

					texture_data[y * (width * 4) + x * 4 + 0] = 255;
					texture_data[y * (width * 4) + x * 4 + 1] = 255;
					texture_data[y * (width * 4) + x * 4 + 2] = 255;
					texture_data[y * (width * 4) + x * 4 + 3] = (v * 255.0) as u8;
				});
			}
		}

		Texture {
			width,
			height,
			data: texture_data
		}
	}
}

fn get_texture_size_from_glyphs(glyphs: &Vec<PositionedGlyph>) -> (usize, usize) {
	let mut min_x = 0;
	let mut min_y = 0;
	let mut max_x = 0;
	let mut max_y = 0;

	for glyph in glyphs.iter() {
		if let Some(bb) = glyph.pixel_bounding_box() {
			min_x = cmp::min(min_x, bb.min.x);
			min_y = cmp::min(min_y, bb.min.y);
			max_x = cmp::max(max_x, bb.max.x);
			max_y = cmp::max(max_y, bb.max.y);
		}
	}

	let width = (max_x - min_x) as usize;
	let height = (max_y - min_y) as usize;

	(width, height)
}