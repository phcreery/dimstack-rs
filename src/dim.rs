use crate::dist;
use crate::stats;
use crate::tol;

use std::f32;

#[derive(Clone, Copy)]
pub enum Direction {
    Positive,
    Negative,
}

impl Direction {
    pub fn signum(&self) -> f32 {
        match self {
            Direction::Positive => 1.0,
            Direction::Negative => -1.0,
        }
    }
    pub fn from(num: f32) -> Direction {
        if num > 0.0 {
            Direction::Positive
        } else {
            Direction::Negative
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vec1D {
    pub x: f32,
}

impl Vec1D {
    pub fn new(x: f32) -> Vec1D {
        Vec1D { x }
    }
}

#[derive(Clone)]
pub struct Basic {
    pub nom: Vec1D,
    pub tol: tol::Bilateral,
    pub a: f32, // sensitivity
    pub name: String,
    pub desc: String,
}

impl Basic {
    /// A Basic geometric 1D measurement.
    /// All measurements are assumed to be 1D linear and relative.
    ///
    /// # Arguments
    ///
    /// * `nom` - Nominal absolute value of the measurement.
    /// * `tol` - Tolerance of the measurement.
    /// * `a` - Sensitivity of the measurement.
    /// * `name` - Name of the measurement.
    /// * `desc` - Description of the measurement.
    ///
    pub fn new(nom: Vec1D, tol: tol::Bilateral, a: f32, name: String, desc: String) -> Basic {
        Basic {
            nom,
            tol,
            a,
            name,
            desc,
        }
    }

    pub fn median(&self) -> f32 {
        return (self.lower() + self.upper()) / 2.0;
    }

    pub fn lower(&self) -> f32 {
        return self.nom.x.abs() + self.tol.lower;
    }

    pub fn upper(&self) -> f32 {
        return self.nom.x.abs() + self.tol.upper;
    }

    pub fn absolute_nominal_adj(&self) -> f32 {
        return self.nom.x * self.a;
    }

    pub fn absolute_median(&self) -> f32 {
        return (self.absolute_lower() + self.absolute_upper()) / 2.0;
    }

    pub fn absolute_median_adj(&self) -> f32 {
        return self.absolute_median() * self.a;
    }

    pub fn absolute_lower(&self) -> f32 {
        return self.nom.x + self.absolute_lower_tol();
    }

    pub fn absolute_upper(&self) -> f32 {
        return self.nom.x + self.absolute_upper_tol();
    }

    pub fn absolute_lower_tol(&self) -> f32 {
        if self.nom.x > 0.0 {
            return self.tol.lower;
        } else {
            return -self.tol.upper;
        }
    }

    pub fn absolute_upper_tol(&self) -> f32 {
        if self.nom.x > 0.0 {
            return self.tol.upper;
        } else {
            return -self.tol.lower;
        }
    }

    pub fn recenter_nom(&self) -> Basic {
        let median = self.median();
        let tol = self.tol.t() / 2.0;

        Basic::new(
            Vec1D::new(median),
            tol::Bilateral::symmetric(tol),
            self.a,
            self.name.clone(),
            self.desc.clone(),
        )
    }

    // todo: move reviewed to its own module and add this as impl
    // pub fn review(&self, target_process_sigma: f32, distribution: Option<dist::Uniform | dist::Normal>) -> Reviewed {
    //     return Reviewed::new(self, target_process_sigma,
    //                          distribution.unwrap_or(dist::Normal::new(0.0, 1.0)));
    // }
}

pub struct Stack {
    pub name: String,
    pub description: String,
    pub dims: Vec<Basic>,
}

impl Stack {
    pub fn new(name: String, description: String, dims: Vec<Basic>) -> Stack {
        Stack {
            name,
            description,
            dims,
        }
    }

    pub fn append(&mut self, measurement: Basic) {
        self.dims.push(measurement);
    }

    pub fn from_reviewed(reviewed_stack: &ReviewedStack) -> Stack {
        let dims = reviewed_stack
            .dims
            .iter()
            .map(|rdim| rdim.dim.clone())
            .collect();
        Stack {
            name: reviewed_stack.name.clone(),
            description: reviewed_stack.description.clone(),
            dims,
        }
    }
}

#[derive(Clone)]
pub struct Reviewed {
    pub dim: Basic,
    pub target_process_sigma: f32,
    // TODO: remove option
    pub distribution: Option<dist::Distribution>,
}

impl Reviewed {
    pub fn new(
        dim: Basic,
        target_process_sigma: f32,
        distribution: Option<dist::Distribution>,
    ) -> Reviewed {
        let mut ret = Reviewed {
            dim,
            target_process_sigma,
            distribution,
        };
        // if distribution is none, assume normal distribution
        if ret.distribution.is_none() {
            ret = ret.assume_normal_dist();
        }
        return ret;
    }

    pub fn of_basic(basic: Basic, target_process_sigma: f32) -> Reviewed {
        Reviewed::new(basic, target_process_sigma, None)
    }

    // pub fn assume_normal_dist(&mut self) -> &mut Reviewed {
    //     let mean = self.mean_eff();
    //     let std_dev = (self.dim.absolute_upper() - self.dim.absolute_lower())
    //         / (2.0 * self.target_process_sigma);
    //     let distribution = dist::Normal::new(mean, std_dev);
    //     self.distribution = Some(dist::Distribution::Normal(distribution));
    //     self
    // }

    pub fn assume_normal_dist(self) -> Reviewed {
        let mean = self.mean_eff();
        let std_dev = (self.dim.absolute_upper() - self.dim.absolute_lower())
            / (2.0 * self.target_process_sigma);
        let distribution = dist::Normal::new(mean, std_dev);
        Reviewed {
            dim: self.dim,
            target_process_sigma: self.target_process_sigma,
            distribution: Some(dist::Distribution::Normal(distribution)),
        }
    }

    // pub fn assume_normal_dist_skewed(&mut self, skew: f32) -> &mut Reviewed {
    //     self.assume_normal_dist();
    //     match &self.distribution {
    //         Some(dist::Distribution::Normal(normal)) => {
    //             let mean = normal.mean + skew * (normal.std_dev * self.target_process_sigma);
    //             let distribution = dist::Normal::new(mean, normal.std_dev);
    //             self.distribution = Some(dist::Distribution::Normal(distribution));
    //         }
    //         _ => {}
    //     }
    //     return self;
    // }

    pub fn assume_normal_dist_skewed(self, skew: f32) -> Reviewed {
        let reviewed = self.assume_normal_dist();
        match &reviewed.distribution {
            Some(dist::Distribution::Normal(normal)) => {
                let mean = normal.mean + skew * (normal.std_dev * reviewed.target_process_sigma);
                let distribution = dist::Normal::new(mean, normal.std_dev);
                Reviewed {
                    dim: reviewed.dim,
                    target_process_sigma: reviewed.target_process_sigma,
                    distribution: Some(dist::Distribution::Normal(distribution)),
                }
            }
            _ => reviewed,
        }
    }

    pub fn c_p(&self) -> f32 {
        // only normal distribution is supported
        match &self.distribution {
            Some(dist::Distribution::Normal(normal)) => {
                stats::c_p(self.dim.upper(), self.dim.lower(), normal.std_dev)
            }
            _ => 0.0,
        }
    }

    pub fn c_pk(&self) -> f32 {
        // only normal distribution is supported
        match &self.distribution {
            Some(dist::Distribution::Normal(normal)) => stats::c_pk(
                self.dim.upper(),
                self.dim.lower(),
                normal.mean,
                normal.std_dev,
            ),
            _ => 0.0,
        }
    }

    pub fn mean_eff(&self) -> f32 {
        (self.dim.absolute_lower() + self.dim.absolute_upper()) / 2.0
    }

    /// Effective standard deviation
    /// https://www.six-sigma-material.com/
    /// "6 std_dev" is the standard deviation of the distribution
    ///
    /// TODO: document source
    pub fn std_dev_eff(&self) -> f32 {
        match &self.distribution {
            Some(dist::Distribution::Normal(normal)) => {
                let outer_shift = f32::min(
                    self.dim.absolute_upper() - normal.mean,
                    normal.mean - self.dim.absolute_lower(),
                );
                return (self.dim.tol.t() * normal.std_dev) / (2.0 * outer_shift);
            }
            _ => 0.0,
        }
    }

    pub fn process_sigma_eff(&self) -> f32 {
        if self.std_dev_eff() == 0.0 {
            return 0.0;
        }
        let min_tol_gap = (self.dim.absolute_upper() - self.mean_eff())
            .min(self.mean_eff() - self.dim.absolute_lower());
        min_tol_gap / self.std_dev_eff()
    }

    pub fn k(&self) -> f32 {
        match &self.distribution {
            Some(dist::Distribution::Normal(normal)) => {
                let outer_shift = f32::min(
                    self.dim.absolute_upper() - normal.mean,
                    normal.mean - self.dim.absolute_lower(),
                );
                return self.dim.tol.t() / (2.0 * outer_shift);
            }
            _ => 0.0,
        }
    }

    pub fn yield_loss_probability(&self) -> f32 {
        if self.distribution.is_none() {
            return 0.0;
        }
        1.0 - self.yield_probability()
    }

    pub fn yield_probability(&self) -> f32 {
        // only normal distribution is supported
        match &self.distribution {
            Some(dist::Distribution::Normal(normal)) => {
                let ul = self.dim.absolute_upper();
                let ll = self.dim.absolute_lower();
                normal.cdf(ul) - normal.cdf(ll)
            }
            _ => 0.0,
        }
    }
}
pub struct ReviewedStack {
    pub name: String,
    pub description: String,
    pub dims: Vec<Reviewed>,
}

impl ReviewedStack {
    pub fn new(name: String, description: String, dims: Vec<Reviewed>) -> ReviewedStack {
        ReviewedStack {
            name,
            description,
            dims,
        }
    }

    pub fn append(&mut self, measurement: Reviewed) {
        self.dims.push(measurement);
    }
}

pub struct Requirement {
    pub name: String,
    pub description: String,
    pub distribution: dist::Distribution,
    pub ll: f32,
    pub ul: f32,
}

impl Requirement {
    pub fn new(
        name: String,
        description: String,
        distribution: dist::Distribution,
        ll: f32,
        ul: f32,
    ) -> Requirement {
        Requirement {
            name,
            description,
            distribution,
            ll,
            ul,
        }
    }

    pub fn median(&self) -> f32 {
        (self.ll + self.ul) / 2.0
    }

    pub fn yield_loss_probability(&self) -> f32 {
        1.0 - self.yield_probability()
    }

    pub fn yield_probability(&self) -> f32 {
        match &self.distribution {
            dist::Distribution::Uniform(uniform) => uniform.cdf(self.ul) - uniform.cdf(self.ll),
            dist::Distribution::Normal(normal) => normal.cdf(self.ul) - normal.cdf(self.ll),
        }
    }

    pub fn r(&self) -> f32 {
        self.yield_loss_probability() * 1000000.0
    }

    // pub fn dict(&self) -> Vec<serde_json::Value> {
    //     vec![serde_json::json!({
    //         "Name": self.name,
    //         "Desc.": self.description,
    //         "Distribution": self.distribution.to_string(),
    //         "Median": self.median(),
    //         "Spec. Limits": [self.ll, self.ul],
    //         "Yield Prob.": self.yield_probability() * 100.0,
    //         "Reject PPM": self.r(),
    //     })]
    // }
}
