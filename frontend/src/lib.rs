use zoon::{*, println};

use vec::{Vec3, Point3, Color};
use ray::Ray;
use camera::Camera;
use rand::Rng;
use rayon::prelude::*;

mod ray;
mod vec;
mod hit;
mod sphere;
mod camera;
mod material;
mod scene;

use scene::scene;

// ------ ------
//    Statics 
// ------ ------

// ------ ------
//    Signals 
// ------ ------

// ------ ------
//   Commands 
// ------ ------

// ------ ------
//     View 
// ------ ------

fn root() -> impl Element {
    Text::new("Hello!")
}

fn main() -> () {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u64 = 600;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 30;
    const MAX_DEPTH: u64 = 50;

    // World
    let world = scene();

    // Camera
    let lookfrom = Point3::new(13.0, 3.0, 3.0);
    let lookat = Point3::new(0.0, 0.5, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(lookfrom,
                          lookat,
                          vup,
                          20.0,
                          ASPECT_RATIO,
                          aperture,
                          dist_to_focus);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j + 1);

        let scanline: Vec<Color> = (0..IMAGE_WIDTH).into_par_iter().map(|i| {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let mut rng = rand::thread_rng();
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_color += Ray::color(&r, &world, MAX_DEPTH);
            }

            pixel_color
        }).collect();

        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
}


// ------ ------
//     Start 
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
