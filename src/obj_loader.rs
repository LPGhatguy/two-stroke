use std::io::{Lines, BufRead};

#[derive(Debug)]
pub enum ObjItem {
	Group {
		name: String,
	},
	Vertex([f32; 3]),
	Triangle([u32; 3]),
}

fn parse_line(line: &str) -> Option<ObjItem> {
	match line.chars().nth(0) {
		Some('v') => {
			let line = &line[1..];
			let mut items = Vec::<f32>::new();

			for item in line.split_whitespace() {
				match item.parse::<f32>() {
					Ok(value) => items.push(value),
					Err(_) => {
						println!("Invalid float {:?}!", item);
					}
				}
			}

			if items.len() < 3 {
				return None;
			}

			let values = [items[0], items[1], items[2]];

			Some(ObjItem::Vertex(values))
		},
		Some('f') => {
			let line = &line[1..];
			let mut items = Vec::<u32>::new();

			for item in line.split_whitespace() {
				match item.parse::<u32>() {
					Ok(value) => items.push(value),
					Err(_) => {
						println!("Invalid usize {:?}!", item);
					}
				}
			}

			if items.len() < 3 {
				return None;
			}

			let values = [items[0] - 1, items[1] - 1, items[2] - 1];

			Some(ObjItem::Triangle(values))
		},
		Some('g') => {
			Some(ObjItem::Group {
				name: line[2..].to_string(),
			})
		},
		Some(val) => {
			println!("Got unknown start {}", val);
			None
		},
		None => None
	}
}

pub struct ObjStream<'a> {
	iterator: Lines<&'a mut BufRead>,
}

impl<'a> ObjStream<'a> {
	pub fn new(reader: &'a mut BufRead) -> ObjStream {
		ObjStream {
			iterator: reader.lines(),
		}
	}
}

impl<'a> Iterator for ObjStream<'a> {
	type Item = ObjItem;

	fn next(&mut self) -> Option<ObjItem> {
		loop {
			match self.iterator.next() {
				Some(line) => {
					match line {
						Ok(line) => {
							match parse_line(line.as_str()) {
								Some(object) => return Some(object),
								None => {},
							}
						},
						Err(_) => return None,
					}
				},
				None => return None,
			}
		}
	}
}