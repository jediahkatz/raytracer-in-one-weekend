use crate::vec3::{Point3, Vec3, Color3};
use crate::ray::Ray;
use crate::hittable::Hit;

pub trait Material {
    fn scatter(&self, r_in: &Ray, h: &Hit) -> (Color3, Ray);
}

pub struct Lambertian {
    pub albedo: Color3,
}

impl Lambertian {
    pub fn new(albedo: Color3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, h: &Hit) -> (Color3, Ray) {
        let scatter_dir: Vec3 = &h.normal + &Vec3::random_unit_vector();
        (self.albedo, Ray::new(&h.p, &scatter_dir))
    }
}