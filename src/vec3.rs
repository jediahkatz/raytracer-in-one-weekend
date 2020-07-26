
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

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl<'a, 'b> std::ops::Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> std::ops::Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, v: Self) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
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

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, v: Self) -> Self {
        Self {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

impl<'a, 'b> std::ops::Mul<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: other.x * self.x,
            y: other.y * self.y,
            z: other.z * self.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    
    fn mul(self, k: f64) -> Self {
        Self {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl<'a> std::ops::Mul<f64> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
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

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    
    fn div(self, k: f64) -> Self {
        Self {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        }
    }
}

impl<'a> std::ops::Div<f64> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: (1.0/other) * self.x,
            y: (1.0/other) * self.y,
            z: (1.0/other) * self.z,
        }
    }
}


impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn mdot(self, v: Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &Self) -> Self {
        Self {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn normalized(&self) -> Self {
        self * (1.0/self.length())
    }
}

pub type Point3 = Vec3;
pub type Color3 = Vec3;


impl Color3 {

    pub fn println(&self) {
        let ir: i32 = (255.999 * self.x) as i32;
        let ig: i32 = (255.999 * self.y) as i32;
        let ib: i32 = (255.999 * self.z) as i32;

        println!("{} {} {}", ir, ig, ib);
    }

}

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
