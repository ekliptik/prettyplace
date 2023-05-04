
#[derive(Clone, Copy, Default)]
pub struct Loc {
    pub x: f32,
    pub y: f32,
}
impl std::ops::Div<Self> for Loc {
    type Output = Loc;
    fn div(self, other: Self) -> Loc {
        return Loc {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}
impl std::ops::Mul<Self> for Loc {
    type Output = Loc;
    fn mul(self, other: Self) -> Loc {
        return Loc {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}
impl std::ops::Mul<f32> for Loc {
    type Output = Loc;
    fn mul(self, other: f32) -> Loc {
        return Loc {
            x: self.x * other,
            y: self.y * other,
        };
    }
}
impl std::ops::Div<f32> for Loc {
    type Output = Loc;
    fn div(self, other: f32) -> Loc {
        return Loc {
            x: self.x / other,
            y: self.y / other,
        };
    }
}
impl std::ops::Add for Loc {
    type Output = Loc;
    fn add(self, other: Loc) -> Loc {
        return Loc {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl std::ops::Sub for Loc {
    type Output = Loc;
    fn sub(self, other: Loc) -> Loc {
        return Loc {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}
impl nannou::prelude::Zero for Loc {
    fn zero() -> Self {
        return Loc { x: 0.0, y: 0.0 };
    }
    fn is_zero(&self) -> bool {
        return self.x == 0.0 && self.y == 0.0;
    }
}
