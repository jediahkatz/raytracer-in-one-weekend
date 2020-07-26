use crate::vec3::{Point3, Vec3, Color3};

#[derive(Debug, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3) -> Self {
        Self { orig: orig.clone(), dir: dir.clone() }
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.orig + &(&self.dir * t)
    }

    pub fn color(&self) -> Color3 {
        match self.hits_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5) {
            Some(t) if t >= 0.0 => {
                let N : Vec3 = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
                Color3::new(N.x + 1.0, N.y + 1.0, N.z + 1.0) * 0.5
            }
            _ => {
                // Linearly blends white and blue depending on y-height
                // of normalized ray direction vector
                let unit_dir: Vec3 = self.dir.normalized();
                let t: f64 = 0.5 * (unit_dir.y + 1.0);
                // Lerp between #FFF (white) and #80B3FF (sky blue)
                Color3::new(1.0, 1.0, 1.0) * (1.0 - t) + Color3::new(0.5, 0.7, 1.0) * t
            }
        }
    }

    /// Returns `t` s.t. this ray intersects the given sphere at `t`, or None
    pub fn hits_sphere(&self, center: &Point3, radius: f64) -> Option<f64> {
        let oc: Vec3 = &self.orig - center;
        let a: f64 = self.dir.dot(&self.dir);
        let b: f64 = 2.0 * oc.dot(&self.dir);
        let c: f64 = oc.dot(&oc) - radius*radius;
        let discriminant: f64 = b*b - 4.0*a*c;
        if discriminant >= 0.0 {
            return Some((-b - discriminant.sqrt()) / (2.0 * a));
        }
        return None;
    }
}