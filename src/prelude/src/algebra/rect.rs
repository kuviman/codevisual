use ::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect<T: Num + Copy> {
    pub bottom_left: Vec2<T>,
    pub top_right: Vec2<T>,
}

impl<T: Num + Copy + PartialOrd> Rect<T> {
    pub fn from_corners(p1: Vec2<T>, p2: Vec2<T>) -> Self {
        let (min_x, max_x) = min_max(p1.x, p2.x);
        let (min_y, max_y) = min_max(p1.y, p2.y);
        Self {
            bottom_left: vec2(min_x, min_y),
            top_right: vec2(max_x, max_y),
        }
    }
    pub fn width(&self) -> T {
        self.top_right.x - self.bottom_left.x
    }
    pub fn height(&self) -> T {
        self.top_right.y - self.bottom_left.x
    }
    pub fn size(&self) -> Vec2<T> {
        vec2(self.width(), self.height())
    }
}