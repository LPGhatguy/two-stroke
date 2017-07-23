use rusttype;
use rusttype::{FontCollection, PositionedGlyph};

pub struct Font<'a> {
	font: rusttype::Font<'a>,
}

impl<'a> Font<'a> {
	pub fn from_bytes(font_data: &[u8]) -> Option<Font> {
		let collection = FontCollection::from_bytes(font_data);

		if let Some(font) = collection.into_font() {
			Some(Font {
				font: font,
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
}