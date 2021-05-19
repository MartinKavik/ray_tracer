mod ray;
mod vec;
mod hit;
mod sphere;
mod camera;
mod material;
mod scene;

use zoon::{*, println};
use vec::{Vec3, Point3, Color};
use ray::Ray;
use camera::Camera;
use rand::Rng;
use scene::scene;

// Image
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: u64 = 600;
const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
const SAMPLES_PER_PIXEL: u64 = 50;
const MAX_DEPTH: u64 = 50;

fn root() -> impl Element {
    RawHtmlEl::new("canvas")
        .attr("id", "canvas")
        .attr("width", &IMAGE_WIDTH.to_string())
        .attr("height", &IMAGE_HEIGHT.to_string())
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);

    let ctx = canvas_ctx();

    let draw_pixel = |x: u64, y: u64, color: String| {
        ctx.set_fill_style(&JsValue::from_str(&color));
        ctx.fill_rect(x as f64, y as f64, 1., 1.);
    };

    draw(draw_pixel);
}

fn canvas_ctx() -> web_sys::CanvasRenderingContext2d {
    document()
        .get_element_by_id("canvas")
        .map(|element| element.unchecked_into::<web_sys::HtmlCanvasElement>())
        .unwrap_throw()
        .get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .unchecked_into::<web_sys::CanvasRenderingContext2d>()
}

fn draw(draw_pixel: impl Fn(u64, u64, String)) -> () {
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

    for j in 0..IMAGE_HEIGHT {
        println!("Remaining lines: {}", IMAGE_HEIGHT - j);

        for i in 0..IMAGE_WIDTH {
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
            draw_pixel(i, IMAGE_HEIGHT - j, pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
}
