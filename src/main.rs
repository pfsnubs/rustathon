fn main() {
    // image
    let image_width = 256;
    let image_height = 256;

    // render
    println!("P3\n{} {}\n255\n", image_width, image_height);
    for i in 0..image_height {
        println!("Scan line {}/image_height", i);
        for j in 0..image_width {
            let r = (j as f64) / (image_width-1) as f64;
            let g = (j as f64) / (image_height-1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            // println!("{} {} {}\n", ir, ig, ib);
        }
    }
    println!("Done!");
}