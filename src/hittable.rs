use crate::vec3::{Point3, Vec3, Color3};
use crate::ray::{Ray};
use crate::material::Material;

pub struct Hit<'a> {
    pub p: Point3,
    /// Normal vector pointing in the opposite direction of the ray
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    /// Whether this intersection is on the front/outside face of the object
    pub front_face: bool
}

impl<'a> Hit<'a> {
    
    pub fn new(p: Point3, t: f64, material: &'a dyn Material,
               r: &Ray, outward_normal: Vec3) -> Hit<'a> {
        let front_face: bool = r.dir.dot(&outward_normal) < 0.0;
        Hit {
            p, t, front_face, material,
            normal: if front_face { outward_normal } else { -outward_normal }
        }
    }

}

pub trait Object {

    /// If this object is hit by the given ray, return a Hit struct
    fn hit_by(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;

}

/// A collection of Hittables is itself Hittable
impl Object for Vec<&dyn Object> {

    fn hit_by(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;

        // Find the closest hit
        for hittable in self {
            let maybe_hit = hittable.hit_by(
                r, t_min, 
                closest_hit.as_ref().and_then(|hit| { Some(hit.t) }).unwrap_or(t_max) // Monads!
            );
            if maybe_hit.is_some() {
                closest_hit = maybe_hit
            }
        }

        closest_hit
    }

}

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    
    pub fn new(center: &Point3, radius: f64, material: &'a dyn Material) -> Self {
        Sphere { center: center.clone(), radius, material }
    }

}

impl<'a> Object for Sphere<'a> {

    fn hit_by(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc: Vec3 = &r.orig - &self.center;
        let a: f64 = r.dir.length_squared();
        let half_b: f64 = oc.dot(&r.dir);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant >= 0.0 {
            let root: f64 = discriminant.sqrt();
            let t: f64 = (-half_b - root) / a;
            if t < t_max && t > t_min {
                let p: Vec3 = r.at(t);
                let outer_normal: Vec3 = (&p - &self.center) / self.radius;
                return Some(Hit::new(
                    p, t, self.material, &r, outer_normal,
                ))
            }

            let t: f64 = (-half_b + root) / a;
            if t < t_max && t > t_min {
                let p: Vec3 = r.at(t);
                let outer_normal: Vec3 = (&p - &self.center) / self.radius;
                return Some(Hit::new(
                    p, t, self.material, &r, outer_normal
                ))
            }
        }
        return None;
    }
}