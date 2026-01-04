#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn new_i32(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn normal(&self) -> Self {
        *self / self.length()
    }

    pub fn color(&self, max_val: u16) -> [u16; 3] {
        [
            (self.x * max_val as f64 + 0.999) as u16,
            (self.y * max_val as f64 + 0.999) as u16,
            (self.z * max_val as f64 + 0.999) as u16,
        ]
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        let recip = 1.0 / rhs;
        Self {
            x: self.x * recip,
            y: self.y * recip,
            z: self.z * recip,
        }
    }
}

impl std::ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
