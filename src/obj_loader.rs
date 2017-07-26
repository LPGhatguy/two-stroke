use std::io::{Lines, BufRead};

#[derive(Debug)]
pub enum ObjItem {
	Vertex,
}

fn parse_line(line: String) -> Option<ObjItem> {
	match line.chars().nth(0) {
		Some('v') => {
			Some(ObjItem::Vertex)
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
							match parse_line(line) {
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