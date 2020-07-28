use crate::hittable::{Object};
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

    pub fn color(&self, world: &Vec<&dyn Object>, depth: i32) -> Color3 {
        if depth <= 0 {
            return Color3::new(0.0, 0.0, 0.0);
        }
        if let Some(hit) = world.hit_by(self, 0.001, f64::INFINITY) {
            if let Some((attenuation, scattered)) = hit.material.scatter(self, &hit) {
                return attenuation * scattered.color(world, depth-1)
            }
            return Color3::new(0.0, 0.0, 0.0)
        }
        let unit_dir: Vec3 = self.dir.normalized();
        let t: f64 = 0.5 * (unit_dir.y + 1.0);
        // Lerp between #FFF (white) and #80B3FF (sky blue)
        Color3::new(1.0, 1.0, 1.0) * (1.0 - t) + Color3::new(0.5, 0.7, 1.0) * t
    }
}