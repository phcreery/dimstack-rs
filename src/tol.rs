#[derive(Clone)]
pub struct Bilateral {
    pub upper: f32,
    pub lower: f32,
}

impl Bilateral {
    pub fn new(upper: f32, lower: f32) -> Bilateral {
        Bilateral::asymmetric(upper, lower)
    }
    /// Create a bilateral tolerance with different upper and lower bounds
    pub fn asymmetric(upper: f32, lower: f32) -> Bilateral {
        // tolerances are always relative to the nominal value
        if upper < lower {
            return Bilateral {
                upper: lower,
                lower: upper,
            };
        } else {
            return Bilateral { upper, lower };
        }
    }

    /// Create a bilateral tolerance where the upper and lower bounds are the same distance from the nominal value
    pub fn symmetric(tol: f32) -> Bilateral {
        Bilateral {
            upper: tol,
            lower: -tol,
        }
    }

    /// Create a bilateral tolerance with different upper and lower bounds
    /// alias for `asymmetric`
    pub fn unequal(upper: f32, lower: f32) -> Bilateral {
        Bilateral::asymmetric(upper, lower)
    }

    pub fn t(&self) -> f32 {
        return self.upper - self.lower;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tol_bilateral() {
        // double positive flipped
        let tol = Bilateral::asymmetric(1.0, 2.0);
        assert_eq!(tol.upper, 2.0);
        assert_eq!(tol.lower, 1.0);
        assert_eq!(tol.t(), 1.0);

        // symmetric
        let tol = Bilateral::symmetric(1.0);
        assert_eq!(tol.upper, 1.0);
        assert_eq!(tol.lower, -1.0);
        assert_eq!(tol.t(), 2.0);

        let tol = Bilateral::unequal(2.0, -1.0);
        assert_eq!(tol.upper, 2.0);
        assert_eq!(tol.lower, -1.0);
        assert_eq!(tol.t(), 3.0);
    }
}
