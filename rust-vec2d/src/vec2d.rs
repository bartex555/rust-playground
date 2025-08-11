
#[derive(PartialEq)]
pub struct Vec2D{
    pub x: f32,
    pub y: f32
}

impl Vec2D{
    pub fn normalize(self) -> Self{
        let length = (self.x.powi(2) + self.y.powi(2)).sqrt();
        Vec2D { x: (self.x/length), y: (self.y/length) }
    }
}


impl std::ops::Add for Vec2D {
    type Output = Self;

    fn add(self, other: Vec2D) -> Self {
        Vec2D { x: (self.x + other.x), y: (self.y + other.y) }
    }
}

impl std::ops::Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Vec2D) -> Self {
        Vec2D { x: (self.x - other.x), y: (self.y - other.y) }
    }
}

impl std::fmt::Display for Vec2D{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}