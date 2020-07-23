mod vec3;

fn main() {

    let img_width: i32 = 256;
    let img_height: i32 = 256;

    println!("P3\n{} {}\n255", img_width, img_height);

    for j in (0..img_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..img_width {
            let r: f64 = f64::from(i)/f64::from(img_height-1);
            let g: f64 = f64::from(j)/f64::from(img_width-1);
            let b: f64 = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{} {} {}", ir, ib, ig);
        }
    }

    eprintln!("Done")
}
