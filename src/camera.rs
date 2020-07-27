use crate::vec3::{Vec3, Point3};
use crate::ray::{Ray};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = ASPECT_RATIO * viewport_height;
        let focal_length: f64 = 1.0;

        let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner: Vec3 = &(&origin - &(&horizontal/2.0)) - &(&vertical/2.0) - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin, horizontal, vertical, lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let origin = &self.origin;
        let lower_left_corner = &self.lower_left_corner;
        let horizontal = &self.horizontal;
        let vertical = &self.vertical;
        // I know this sucks and could be avoided by defining &u + v and u + &v, but I want to practice understanding ownership
        Ray::new(origin, &(lower_left_corner + &(&(horizontal * u) + &(&(vertical * v) - origin))))
    }
}