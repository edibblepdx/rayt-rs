use indicatif::ProgressIterator;
//use log::info;

fn main() {
    env_logger::init();

    // Image

    let image_width: usize = 256;
    let image_height: usize = 256;

    let scale_x = 1.0 / (image_width - 1) as f64;
    let scale_y = 1.0 / (image_height - 1) as f64;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let r = i as f64 * scale_x;
            let g = j as f64 * scale_y;
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            println!("{ir} {ig} {ib}")
        }
    }
}
