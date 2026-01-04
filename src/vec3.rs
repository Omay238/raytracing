use rand::random;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

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
    pub fn random() -> Self {
        loop {
            let p = Self::new(
                random::<f64>() * 2.0 - 1.0,
                random::<f64>() * 2.0 - 1.0,
                random::<f64>() * 2.0 - 1.0,
            );
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Self) -> Vec3 {
        let p = Self::random();
        if normal.dot(&p) > 0.0 { p } else { -p }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn normal(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflected(&self, normal: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(normal) * *normal
    }

    pub fn refracted(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }

    pub fn color(&self, max_val: u16) -> [u16; 3] {
        [
            (self.x * max_val as f64 + 0.999) as u16,
            (self.y * max_val as f64 + 0.999) as u16,
            (self.z * max_val as f64 + 0.999) as u16,
        ]
    }

    pub fn color_correct(&mut self) {
        self.x = self.x.clamp(0.0, 1.0).sqrt();
        self.y = self.y.clamp(0.0, 1.0).sqrt();
        self.z = self.z.clamp(0.0, 1.0).sqrt();
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

impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
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
