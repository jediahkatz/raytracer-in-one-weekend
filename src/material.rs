use crate::vec3::{Point3, Vec3, Color3};
use crate::ray::Ray;
use crate::hittable::Hit;

pub trait Material {
    fn scatter(&self, incoming: &Ray, h: &Hit) -> Option<(Color3, Ray)>;
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
    fn scatter(&self, _incoming: &Ray, h: &Hit) -> Option<(Color3, Ray)> {
        let scatter_dir: Vec3 = &h.normal + &Vec3::random_unit_vector();
        Some((self.albedo.clone(), Ray::new(&h.p, &scatter_dir)))
    }
}

pub struct Metal {
    pub albedo: Color3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color3, fuzz: f64) -> Metal {
        Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, incoming: &Ray, h: &Hit) -> Option<(Color3, Ray)> {
        let reflected: Vec3 = incoming.dir.normalized().reflect(&h.normal);
        let mut scattered: Ray = Ray::new(&h.p, &reflected);
        scattered.dir += Vec3::random_unit_vector() * self.fuzz;
        if scattered.dir.dot(&h.normal) > 0.0 {
            return Some((self.albedo.clone(), scattered))
        }
        None
    }
}