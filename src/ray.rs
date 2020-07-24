mod vec3;
use vec3::Point3;

#[derive(Debug, Clone)]
struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) {
        self.orig + t * self.dir
    }
}