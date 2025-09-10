use rand::Rng;

pub trait Sampler {
    fn samples(&self, x: f64, y: f64) -> Box<dyn Iterator<Item = (f64, f64)>>;
    fn nsamples(&self) -> usize;
}

#[derive(serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum SamplerConfig {
    Random { samples_per_pixel: usize },
    Single,
}

impl SamplerConfig {
    pub fn into_sampler(self) -> Box<dyn Sampler + Sync + 'static> {
        match self {
            SamplerConfig::Random { samples_per_pixel } => {
                Box::new(RandomSampler { samples_per_pixel })
            }
            SamplerConfig::Single => Box::new(SingleSampler {}),
        }
    }
}

/// Randomly samples within a box.
pub struct RandomSampler {
    pub samples_per_pixel: usize,
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

/// Single sample pixel center.
pub struct SingleSampler;

impl Sampler for SingleSampler {
    fn samples(&self, x: f64, y: f64) -> Box<dyn Iterator<Item = (f64, f64)>> {
        Box::new((0..1).map(move |_| (x, y)))
    }

    fn nsamples(&self) -> usize {
        1
    }
}

mod test {
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
}
