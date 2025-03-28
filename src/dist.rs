use statrs::function::erf::erf;
use std::f32::consts::PI;

// TODO: https://docs.rs/statrs/latest/statrs/distribution/index.html
#[derive(Clone)]
pub enum Distribution {
    Uniform(Uniform),
    Normal(Normal),
}
#[derive(Clone)]
pub struct Uniform {
    pub upper: f32,
    pub lower: f32,
}

impl Uniform {
    pub fn new(lower: f32, upper: f32) -> Uniform {
        Uniform { lower, upper }
    }

    pub fn pdf(&self, x: f32) -> f32 {
        if x < self.lower || x > self.upper {
            return 0.0;
        }
        return 1.0 / (self.upper - self.lower);
    }

    pub fn cdf(&self, x: f32) -> f32 {
        if x < self.lower {
            return 0.0;
        } else if x > self.upper {
            return 1.0;
        }
        return (x - self.lower) / (self.upper - self.lower);
    }
}
// https://docs.rs/statrs/latest/statrs/distribution/struct.Normal.html
#[derive(Clone)]
pub struct Normal {
    pub mean: f32,
    pub std_dev: f32,
}

impl Normal {
    pub fn new(mean: f32, std_dev: f32) -> Normal {
        Normal { mean, std_dev }
    }

    pub fn variance(&self) -> f32 {
        return self.std_dev * self.std_dev;
    }

    /// https://people.sc.fsu.edu/~jburkardt/c_src/prob/prob.c
    pub fn pdf(&self, x: f32) -> f32 {
        return 1.0 / (self.std_dev * (2.0 * PI).sqrt())
            * (-0.5 * f32::powf((x - self.mean) / self.std_dev, 2.0)).exp();
    }

    pub fn cdf(&self, x: f32) -> f32 {
        return 0.5
            * (1.0 + erf(((x - self.mean) / (self.std_dev * f32::sqrt(2.0))) as f64)) as f32;
    }

    pub fn fit(measurements: &Vec<f32>) -> Normal {
        let mean = measurements.iter().sum::<f32>() / measurements.len() as f32;
        let variance = measurements
            .iter()
            .map(|x| f32::powf(x - mean, 2.0))
            .sum::<f32>()
            / measurements.len() as f32;
        let std_dev = variance.sqrt();
        return Normal { mean, std_dev };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dist_uniform() {
        let dist = Uniform::new(1.0, 2.0);
        assert_eq!(dist.pdf(0.0), 0.0);
        // assert_eq!(dist.pdf(1.5), 0.5);
        assert_eq!(dist.pdf(2.5), 0.0);
        assert_eq!(dist.cdf(0.0), 0.0);
        assert_eq!(dist.cdf(1.5), 0.5);
        assert_eq!(dist.cdf(2.5), 1.0);
    }
}
