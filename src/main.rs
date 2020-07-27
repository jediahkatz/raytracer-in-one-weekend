mod vec3;
mod ray;
mod hittable;
mod camera;
use vec3::{Color3, Point3, Vec3};
use ray::Ray;
use hittable::{Hittable, Sphere};
use camera::{Camera};

fn main() {

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 400;
    const IMG_HEIGHT: i32 = ((IMG_WIDTH as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    // World
    let mut world: Vec<&dyn Hittable> = Vec::new();
    let s1 : Sphere = Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5);
    let s2 : Sphere = Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0);
    world.push(&s1);
    world.push(&s2);

    // Camera
    let cam: Camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let u: f64 = f64::from(i)/f64::from(IMG_WIDTH-1);
            let v: f64 = f64::from(j)/f64::from(IMG_HEIGHT-1);
            let r: Ray = cam.get_ray(u, v);
            r.color(&world).println(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done")
}
