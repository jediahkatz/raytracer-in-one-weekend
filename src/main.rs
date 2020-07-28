mod vec3;
mod ray;
mod hittable;
mod material;
mod camera;
use vec3::{Color3, Point3, Vec3};
use ray::Ray;
use hittable::{Object, Sphere};
use material::{Lambertian, Metal};
use camera::Camera;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 400;
    const IMG_HEIGHT: i32 = ((IMG_WIDTH as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mat_ground = Lambertian::new(Color3::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Color3::new(0.7, 0.3, 0.3));
    let mat_left = Metal::new(Color3::new(0.8, 0.8, 0.8), 0.3);
    let mat_right = Metal::new(Color3::new(0.8, 0.6, 0.2), 1.0);
    let s_center = Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5, &mat_center);
    let s_ground = Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0, &mat_ground);
    let s_left = Sphere::new(&Point3::new(-1.0, 0.0, -1.0), 0.5, &mat_left);
    let s_right = Sphere::new(&Point3::new(1.0, 0.0, -1.0), 0.5, &mat_right);

    let world: Vec<&dyn Object> = vec![&s_center, &s_ground, &s_left, &s_right];

    // Camera
    let cam: Camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let mut c: Color3 = Color3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (f64::from(i) + rng.gen::<f64>())/f64::from(IMG_WIDTH-1);
                let v: f64 = (f64::from(j) + rng.gen::<f64>())/f64::from(IMG_HEIGHT-1);
                let r: Ray = cam.get_ray(u, v);
                c += r.color(&world, MAX_DEPTH);
            }
            c.println(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done")
}
