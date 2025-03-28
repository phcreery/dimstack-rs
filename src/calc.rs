use crate::dim;
use crate::dim::{Basic, Reviewed, ReviewedStack, Stack};
use crate::dist;
use crate::stats;
use crate::tol::Bilateral;

pub fn closed(stack: &Stack) -> Basic {
    let nominal: f32 = stack
        .dims
        .iter()
        .map(|dim| dim.absolute_nominal_adj())
        .sum();
    let tolerance: Bilateral = if nominal < 0.0 {
        Bilateral::unequal(
            -stack
                .dims
                .iter()
                .map(|dim| dim.absolute_lower_tol())
                .sum::<f32>(),
            -stack
                .dims
                .iter()
                .map(|dim| dim.absolute_upper_tol())
                .sum::<f32>(),
        )
    } else {
        Bilateral::unequal(
            stack.dims.iter().map(|dim| dim.absolute_upper_tol()).sum(),
            stack.dims.iter().map(|dim| dim.absolute_lower_tol()).sum(),
        )
    };
    Basic {
        nom: dim::Vec1D::new(nominal),
        tol: tolerance,
        a: 1.0,
        name: format!("{} - Closed Analysis", stack.name),
        desc: "".to_owned(),
    }
}

/// This is a simple WC calculation.
/// This results in a Bilateral dimension with a tolerance that is the sum of
/// the component tolerances. It states that in any combination of tolerances,
/// you can be sure the result will be within the this resulting tolerance.
pub fn wc(stack: &Stack) -> Basic {
    let mean: f32 = stack.dims.iter().map(|dim| dim.absolute_median_adj()).sum();
    let t_wc: f32 = stack
        .dims
        .iter()
        .map(|dim| ((dim.tol.t() / 2.0) * dim.a).abs())
        .sum();
    let tolerance = Bilateral::symmetric(t_wc);
    Basic {
        nom: dim::Vec1D::new(mean),
        tol: tolerance,
        a: 1.0,
        name: format!("{} - WC Analysis", stack.name),
        desc: "".to_owned(),
    }
}

/// This is a simple RSS calculation. This is uses the RSS calculation method in
/// the Dimensioning and Tolerancing Handbook, McGraw Hill.
/// It is really only useful for a Bilateral stack of same process-std_dev dims.
/// The RSS result has the same uncertainty as /the measurements.
/// Historically, Eq. (9.11) assumed that all of the component tolerances (t_i)
/// represent a 3sigma value for their manufacturing processes. Thus, if all
/// the component distributions are assumed to be normal, then the probability
/// that a dimension is between ±t_i is 99.73%. If this is true, then the
/// assembly gap distribution is normal and the probability that it is ±t_rss
/// between is 99.73%. Although most people have assumed a value of ±3s for
/// piece-part tolerances, the RSS equation works for “equal s” values.
/// If the designer assumed that the input tolerances were ±4s values for the
/// piece-part manufacturing processes, then the probability that the assembly
/// is between ±t_rss is 99.9937 (4s). The 3s process limits using the RSS
/// Model are similar to the Worst Case Model. The minimum gap is equal to the
/// mean value minus the RSS variation at the gap. The maximum gap is equal to
/// the mean value plus the RSS variation at the gap.
///
/// See:
///     - Dimensioning and Tolerancing Handbook, McGraw Hill
///     - http://files.engineering.com/getfile.aspx?folder=69759f43-e81a-4801-9090-a0c95402bfc0&file=RSS_explanation.GIF
pub fn rss(stack: &Stack) -> Basic {
    let d_g: f32 = stack.dims.iter().map(|dim| dim.absolute_median_adj()).sum();
    let t_rss: f32 = stats::rss(
        &stack
            .dims
            .iter()
            .map(|dim| dim.nom.x.signum() * (dim.tol.t() / 2.0) * dim.a)
            .collect(),
    );
    let tolerance = Bilateral::symmetric(t_rss);
    Basic {
        nom: dim::Vec1D::new(d_g),
        tol: tolerance,
        a: 1.0,
        name: format!("{} - RSS Analysis", stack.name),
        desc: "(assuming inputs with Normal Dist. & uniform SD)".to_owned(),
    }
}

/// Basically RSS with a coefficient modifier that makes the tolerance tighter.
pub fn mrss(stack: &Stack) -> Basic {
    let d_g: f32 = stack.dims.iter().map(|dim| dim.absolute_median_adj()).sum();
    let t_wc: f32 = stack
        .dims
        .iter()
        .map(|dim| ((dim.tol.t() / 2.0) * dim.a).abs())
        .sum();
    let t_rss: f32 = stats::rss(
        &stack
            .dims
            .iter()
            .map(|dim| dim.nom.x.signum() * dim.a * (dim.tol.t() / 2.0))
            .collect(),
    );
    let n: f32 = stack.dims.len() as f32;
    let c_f: f32 = (0.5 * (t_wc - t_rss)) / (t_rss * (n.sqrt() - 1.0)) + 1.0;
    let t_mrss: f32 = c_f * t_rss;
    let tolerance = Bilateral::symmetric(t_mrss);
    Basic {
        // dir,
        // nom: d_g,
        nom: dim::Vec1D::new(d_g),
        tol: tolerance,
        a: 1.0,
        name: format!("{} - MRSS Analysis", stack.name),
        desc: "(assuming inputs with Normal Dist. & uniform SD)".to_owned(),
    }
}

pub fn six_sigma(stack: &ReviewedStack, at: f32) -> Reviewed {
    let mean: f32 = stack
        .dims
        .iter()
        .map(|rdim| rdim.dim.absolute_median_adj())
        .sum();
    let std_dev: f32 = stats::rss(&stack.dims.iter().map(|dim| dim.std_dev_eff()).collect());
    let tolerance = Bilateral::symmetric(std_dev * at);
    let dist = dist::Normal::new(mean, std_dev);
    Reviewed {
        dim: Basic {
            nom: dim::Vec1D::new(mean),
            tol: tolerance,
            a: 1.0,
            name: format!("{} - '6 Sigma' Analysis", stack.name),
            desc: "(assuming inputs with Normal Dist.)".to_owned(),
        },
        target_process_sigma: at,
        distribution: Some(dist::Distribution::Normal(dist)),
    }
}
