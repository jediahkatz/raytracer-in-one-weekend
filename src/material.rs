use crate::vec3::{Point3, Vec3, Color3};
use crate::ray::Ray;
use crate::hittable::Hit;

pub trait Material {
    fn scatter(&self, r_in: &Ray, h: &Hit) -> Option<(Color3, Ray)>;
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
    fn scatter(&self, r_in: &Ray, h: &Hit) -> Option<(Color3, Ray)> {
        let scatter_dir: Vec3 = &h.normal + &Vec3::random_unit_vector();
        Some((self.albedo.clone(), Ray::new(&h.p, &scatter_dir)))
    }
}

pub struct Metal {
    pub albedo: Color3,
}

impl Metal {
    pub fn new(albedo: Color3) -> Metal {
        Metal {albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, h: &Hit) -> Option<(Color3, Ray)> {
        let reflected: Vec3 = r_in.dir.normalized().reflect(&h.normal);
        let scattered: Ray = Ray::new(&h.p, &reflected);
        if scattered.dir.dot(&h.normal) > 0.0 {
            return Some((self.albedo.clone(), scattered))
        }
        None
    }
}