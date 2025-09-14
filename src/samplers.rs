use rand::Rng;

pub trait Sampler {
    fn samples(&self, x: f64, y: f64) -> Box<dyn Iterator<Item = (f64, f64)>>;
    fn nsamples(&self) -> usize;
}

#[derive(serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SamplerConfig {
    Single,
    Random { samples_per_pixel: usize },
    Stratified { nx: usize, ny: usize },
}

impl SamplerConfig {
    pub fn into_sampler(self) -> Box<dyn Sampler + Sync + 'static> {
        match self {
            SamplerConfig::Single => Box::new(SingleSampler {}),
            SamplerConfig::Random { samples_per_pixel } => {
                Box::new(RandomSampler { samples_per_pixel })
            }
            SamplerConfig::Stratified { nx, ny } => Box::new(StratifiedSampler { nx, ny }),
        }
    }
}

/// Single sample pixel center.
pub struct SingleSampler;

impl Sampler for SingleSampler {
    fn samples(&self, x: f64, y: f64) -> Box<dyn Iterator<Item = (f64, f64)>> {
        Box::new([(x, y)].into_iter())
    }

    fn nsamples(&self) -> usize {
        1
    }
}

/// Randomly samples within a bounding box.
pub struct RandomSampler {
    samples_per_pixel: usize,
}

impl Sampler for RandomSampler {
    fn samples(&self, x: f64, y: f64) -> Box<dyn Iterator<Item = (f64, f64)>> {
        let mut rng = rand::rng();
        Box::new((0..self.samples_per_pixel).map(move |_| {
            let dx = rng.random_range(-0.5..=0.5);
            let dy = rng.random_range(-0.5..=0.5);
            (x + dx, y + dy)
        }))
    }

    fn nsamples(&self) -> usize {
        self.samples_per_pixel
    }
}

/// Stratified sampler.
///
/// Divides a bounding box into smaller sub-areas and randomly samples
/// once in each sub-area.
pub struct StratifiedSampler {
    nx: usize,
    ny: usize,
}

impl Sampler for StratifiedSampler {
    fn samples(&self, x: f64, y: f64) -> Box<dyn Iterator<Item = (f64, f64)>> {
        let mut rng = rand::rng();

        let sdx = 1.0 / self.nx as f64;
        let sdy = 1.0 / self.ny as f64;

        let sample_range = (sdx / 2.0, sdy / 2.0);
        let sample00_loc = (x - 0.5 + sample_range.0, y - 0.5 + sample_range.1);

        let mut samples = Vec::with_capacity(self.nx * self.ny);
        for i in 0..self.nx {
            for j in 0..self.ny {
                let dx = rng.random_range(-sample_range.0..=sample_range.0);
                let dy = rng.random_range(-sample_range.1..=sample_range.1);

                samples.push((
                    sample00_loc.0 + (i as f64 * sdx) + dx,
                    sample00_loc.1 + (j as f64 * sdy) + dy,
                ));
            }
        }

        Box::new(samples.into_iter())
    }

    fn nsamples(&self) -> usize {
        self.nx * self.ny
    }
}

mod test {
    #[test]
    fn deserialize_single_sampler() {
        use super::*;
        use serde::Deserialize;

        let toml_str = r#"
            [camera]

            [camera.sampler]
            type = "single"
        "#;

        #[derive(Deserialize)]
        struct Camera {
            sampler: SamplerConfig,
        }

        #[derive(Deserialize)]
        struct Config {
            camera: Camera,
        }

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(SamplerConfig::Single == config.camera.sampler);
    }

    #[test]
    fn deserialize_random_sampler() {
        use super::*;
        use serde::Deserialize;

        let toml_str = r#"
            [camera]

            [camera.sampler]
            type = "random"
            samples_per_pixel = 50
        "#;

        #[derive(Deserialize)]
        struct Camera {
            sampler: SamplerConfig,
        }

        #[derive(Deserialize)]
        struct Config {
            camera: Camera,
        }

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(
            SamplerConfig::Random {
                samples_per_pixel: 50
            } == config.camera.sampler
        );
    }

    #[test]
    fn deserialize_stratified_sampler() {
        use super::*;
        use serde::Deserialize;

        let toml_str = r#"
            [camera]

            [camera.sampler]
            type = "stratified"
            nx = 10
            ny = 10
        "#;

        #[derive(Deserialize)]
        struct Camera {
            sampler: SamplerConfig,
        }

        #[derive(Deserialize)]
        struct Config {
            camera: Camera,
        }

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(SamplerConfig::Stratified { nx: 10, ny: 10 } == config.camera.sampler);
    }
}
