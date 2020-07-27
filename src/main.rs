mod vec3;
mod ray;
mod hittable;
use vec3::{Color3, Point3, Vec3};
use ray::Ray;
use hittable::{Hittable, Sphere};

fn main() {

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 400;
    const IMG_HEIGHT: i32 = ((IMG_WIDTH as f64) / ASPECT_RATIO) as i32;

    // World
    let mut world: Vec<&dyn Hittable> = Vec::new();
    let s1 : Sphere = Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5);
    let s2 : Sphere = Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0);
    world.push(&s1);
    world.push(&s2);

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Vec3 = &(&origin - &(&horizontal/2.0)) - &(&vertical/2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let u: f64 = f64::from(i)/f64::from(IMG_WIDTH-1);
            let v: f64 = f64::from(j)/f64::from(IMG_HEIGHT-1);
            // I know this sucks and could be avoided by defining &u + v and u + &v, but I want to practice understanding ownership
            let r: Ray = Ray::new(&origin, &(&lower_left_corner + &(&(&horizontal * u) + &(&(&vertical * v) - &origin))));
            r.color(&world).println();
        }
    }

    eprintln!("Done")
}
