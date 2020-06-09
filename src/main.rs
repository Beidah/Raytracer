use std::error::Error;

use camera::Camera;
use hittable::Hittable;
use ray::Ray;
use raytracer::*;
use vec3::{Color, Vec3};

fn write_color(color: Vec3, samples_per_pixel: i32) -> (u8, u8, u8) {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    (
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    )
}

fn ray_color<T: Hittable>(ray: Ray, background: Color, world: &T, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(record) = world.hit(ray, 0.001, f64::MAX) {
        let mut scattered = Default::default();
        let mut attenuation = Default::default();
        let emitted = record.mat_ptr.emitted(record.u, record.v, record.p);

        if record
            .mat_ptr
            .scatter(&ray, &record, &mut attenuation, &mut scattered)
        {
            return emitted + attenuation * ray_color(scattered, background, world, depth - 1);
        } else {
            return emitted;
        }
    }

    background
}

fn main() -> Result<(), Box<dyn Error>> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 50;
    let max_depth = 30;

    // let world: HittableList = random_scene();
    let world = simple_light();

    let lookfrom = Vec3::new(26.0, 3.0, 6.0);
    let lookat = Vec3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperature = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        aspect_ratio,
        aperature,
        focus_dist,
        0.0,
        1.0,
    );

    let mut last_y = 0;
    let image_buffer = image::ImageBuffer::from_fn(image_width, image_height, |x, y| {
        if y > last_y {
            last_y = y;
            println!("Scanline {} done!", last_y);
        }

        let mut pixel_color = Vec3::default();
        for _ in 0..samples_per_pixel {
            let u = (x as f64 + random_double()) / (image_width as f64 - 1.0);
            let v = 1.0 - (y as f64 + random_double()) / (image_height as f64 - 1.0);

            let ray = camera.get_ray(u, v);
            pixel_color += ray_color(ray, Vec3(0.0, 0.0, 0.0), &world, max_depth);
        }
        let (r, g, b) = write_color(pixel_color, samples_per_pixel);
        image::Rgb([r, g, b])
    });

    // for j in 0..image_height {
    //     println!("\rScanlines remaining: {} ", image_height - j);
    //     for i in 0..image_width {
    //         let mut pixel_color = Vec3::default();
    //         for _ in 0..samples_per_pixel {
    //             let u = (i as f64 + random_double()) / (image_width as f64 - 1.0);
    //             let v = (j as f64 + random_double()) / (image_height as f64 - 1.0);

    //             let ray = camera.get_ray(u, v);
    //             pixel_color += ray_color(ray, &world, max_depth);
    //         }
    //         let pixel = write_color(pixel_color, samples_per_pixel);
    //         pixels.push(pixel.0);
    //         pixels.push(pixel.1);
    //         pixels.push(pixel.2);
    //     }
    // }

    let args: Vec<String> = std::env::args().collect();

    match args.get(1) {
        Some(path) => {
            println!("Saving to: {}", path);
            image_buffer.save(path)?;
        }
        None => {
            println!("Saving to: image.png");
            image_buffer.save("image.png")?;
        }
    }

    Ok(())
}
