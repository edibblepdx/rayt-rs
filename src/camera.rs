use crate::color::*;
use crate::prelude::*;
use crate::samplers::*;

use std::{
    io::{self, Write},
    sync::Arc,
};

use indicatif::{ParallelProgressIterator, ProgressIterator, ProgressStyle};
use rayon::prelude::*;

#[allow(unused)]
pub struct Camera {
    sampler: Box<dyn Sampler + Sync>,
    image_width: usize,
    image_height: usize,
    position: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    forward: UnitVec3,
    right: UnitVec3,
    up: UnitVec3,
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }

    pub fn render(self, world: HittableList) {
        let world = Arc::new(world);
        let image_area = self.image_width * self.image_height;

        let ps = ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7}",
        )
        .unwrap()
        .progress_chars("#>-");

        log::info!("Rendering Image");

        let mut pixels: Vec<Color> = Vec::with_capacity(image_area);
        pixels
            .spare_capacity_mut() // MaybeUninit
            .par_chunks_mut(self.image_width)
            .progress_with_style(ps.clone())
            .enumerate()
            .for_each(|(j, row)| {
                for (i, pixel) in row.iter_mut().enumerate().take(self.image_width) {
                    let mut color = Color::BLACK;
                    for sample in self.sampler.samples(i as f64, j as f64) {
                        let ray = self.get_ray(sample);
                        *color += *self.ray_color(&ray, &world);
                    }

                    let nsamples = self.sampler.nsamples() as f64;
                    let color: Color = color.map(|e| e / nsamples).into();
                    pixel.write(color);
                }
            });

        // Safety: all elements are initialized.
        unsafe { pixels.set_len(image_area) };

        log::info!("Writing Image");

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let mut out = io::BufWriter::new(io::stdout());
        for &pixel_color in pixels.iter().progress_with_style(ps.clone()) {
            write_color(&mut out, pixel_color).expect("Failed Write");
        }
        out.flush().unwrap();
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        if let Some(record) = world.hit(ray, (0.0, INFINITY).into()) {
            let mapped = record.normal.map(|e| (e + 1.0) / 2.0);
            return Color(mapped);
        }

        let mut t = ray.direction().y;
        t = (t + 1.0) / 2.0;

        let start = Color(Vec3::new(1.0, 1.0, 1.0));
        let end = Color(Vec3::new(0.5, 0.7, 1.0));

        Color((1.0 - t) * start.0 + t * end.0)
    }

    #[rustfmt::skip]
    fn get_ray(&self, (sx, sy): (f64, f64)) -> Ray {
        let pixel_sample = self.pixel00_loc
            + (sx * self.pixel_delta_u)
            + (sy * self.pixel_delta_v);

        let ray_direction =
            UnitVec3::new_normalize(pixel_sample - self.position);

        Ray::new(self.position, ray_direction)
    }
}

impl Default for Camera {
    fn default() -> Camera {
        CameraBuilder::default().build()
    }
}

#[derive(serde::Deserialize)]
pub struct CameraBuilder {
    sampler: SamplerConfig,
    aspect_ratio: f64,
    image_width: usize,
    position: Point3,
    look_at: Vec3,
    up: Vec3,
}

impl CameraBuilder {
    pub fn build(self) -> Camera {
        // Camera basis.
        let forward = UnitVec3::new_normalize(self.look_at - self.position);
        let right = UnitVec3::new_normalize(forward.cross(self.up));
        let up = UnitVec3::new_normalize(right.cross(*forward));

        // Image Dimensions.
        let image_width = self.image_width;
        let mut image_height: usize = (image_width as f64 / self.aspect_ratio) as usize;
        image_height = if image_height < 1 { 1 } else { image_height };

        // Viewport Dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;

        // Viewport uv
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Pixel delta uv
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Position of upper left pixel.
        let viewport_upper_left =
            self.position - Vec3::new(viewport_width / 2.0, -viewport_height / 2.0, focal_length);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Sampler.
        let sampler = self.sampler.into_sampler();

        Camera {
            sampler,
            image_width,
            image_height,
            position: self.position,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            forward,
            right,
            up,
        }
    }

    pub fn sampler(mut self, sampler: SamplerConfig) -> Self {
        self.sampler = sampler;
        self
    }

    pub fn aspect_ratio(mut self, aspect: impl Into<f64>) -> Self {
        self.aspect_ratio = aspect.into();
        self
    }

    pub fn image_width(mut self, width: impl Into<usize>) -> Self {
        self.image_width = width.into();
        self
    }

    pub fn position(mut self, pos: impl Into<Vec3>) -> Self {
        self.position = pos.into();
        self
    }

    pub fn look_at(mut self, look_at: impl Into<Vec3>) -> Self {
        self.look_at = look_at.into();
        self
    }

    pub fn up(mut self, vup: impl Into<Vec3>) -> Self {
        self.up = vup.into();
        self
    }
}

impl Default for CameraBuilder {
    fn default() -> CameraBuilder {
        CameraBuilder {
            sampler: SamplerConfig::Single,
            aspect_ratio: 1.0,
            image_width: 100,
            position: Point3::splat(0.0),
            look_at: Vec3::NEG_Z,
            up: Vec3::Y,
        }
    }
}

mod tests {

    #[test]
    fn deserialize() {
        use super::*;
        use serde::Deserialize;

        let toml_str = r#"
            [camera]
            aspect_ratio = 1.5
            image_width = 200
            position = [1.0, 1.0, 1.0]
            look_at = [0.0, 0.0, -2.0]
            up = [0.0, 2.0, 0.0]

            [camera.sampler]
            type = "single"
        "#;

        #[derive(Deserialize)]
        struct Config {
            camera: CameraBuilder,
        }

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(1.5 == config.camera.aspect_ratio);
        assert!(200 == config.camera.image_width);
        assert!(Point3::splat(1.0) == config.camera.position);
        assert!(Vec3::new(0.0, 0.0, -2.0) == config.camera.look_at);
        assert!(Vec3::new(0.0, 2.0, 0.0) == config.camera.up);
    }
}
