#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UPoint {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}
