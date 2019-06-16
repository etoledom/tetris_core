use super::geometry::{Point, Rect, Size};
use super::graphics::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
	pub rect: Rect,
	pub color: Color,
}

impl Block {
	pub fn new(x: i32, y: i32, height: usize, width: usize, color: Color) -> Block {
		return Block {
			rect: Rect {
				origin: Point { x, y },
				size: Size { height, width },
			},
			color,
		};
	}

	pub fn size(&self) -> Size {
		return self.rect.size.clone();
	}

	pub fn position(&self) -> Point {
		return self.rect.origin.clone();
	}
}
