#[derive(Debug, Default, Clone, PartialEq, Copy)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Into<Vec2> for (f32, f32) {
    fn into(self) -> Vec2 {
        Vec2 {
            x: self.0,
            y: self.1,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Copy)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Into<Vec4> for (f32, f32, f32, f32) {
    fn into(self) -> Vec4 {
        Vec4 {
            x: self.0,
            y: self.1,
            z: self.2,
            w: self.3,
        }
    }
}
