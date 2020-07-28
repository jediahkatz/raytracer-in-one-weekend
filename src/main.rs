mod vec3;
mod ray;
mod hittable;
mod material;
mod camera;
use vec3::{Color3, Point3, Vec3};
use ray::Ray;
use hittable::{Object, Sphere};
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
    let mut world: Vec<&dyn Object> = Vec::new();
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
