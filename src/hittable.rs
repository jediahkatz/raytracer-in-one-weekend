struct Hit {
    pub p: Point3;
    pub normal: Vec3;
    pub t: double;
}

trait Hittable {

    /// If this object is hit by the given ray, return a Hit struct
    pub fn hits(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;

}

struct Sphere {
    pub center: Point3;
    pub radius: f64;
}

impl Sphere {
    
    pub fn new(&center, radius) {
        Sphere { center.clone(), radius }
    }

}

impl Hittable for Sphere {
    pub fn hits(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc: Vec3 = &r.orig - &self.center;
        let a: f64 = r.dir.length_squared();
        let half_b: f64 = oc.dot(&r.dir);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant >= 0.0 {
            let root: f64 = discriminant.sqrt();
            let t: f64 = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let p: r.at(temp);
                return Some(Hit {
                    t, p,
                    normal: (p - self.center) / self.radius
                })
            }

            let t: f64 = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let p: r.at(temp);
                return Some(Hit {
                    t, p,
                    normal: (p - self.center) / self.radius
                })
            }
        }
        return None;
    }
}