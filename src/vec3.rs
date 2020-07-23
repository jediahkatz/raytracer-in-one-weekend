#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, v: Self) {
        *self = Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, k: f64) {
        *self = Self {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, k: f64) {
        *self = Self {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        }
    }
}

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

type Point3 = Vec3;
type Color3 = Vec3;


// impl std::ops::Index<i32> for Vec3 {
//     type Output = f64;
    
//     fn index(&self, i: i32) -> f64 {
//         match i {
//             0 => self.x,
//             1 => self.y,
//             2 => self.z,
//             _ => panic!("Vec3 index out of bounds")
//         }
//     }
// }

// impl std::ops::IndexMut<i32> for Vec3 {
//     type Output = f64;

//     fn index_mut(&mut self, i: i32) -> f64 {
//         match i {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             2 => &mut self.z,
//             _ => panic!("Vec3 index out of bounds")
//         }
//     }
// }
