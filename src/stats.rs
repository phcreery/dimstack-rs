/// Process capability index.
///
/// # Arguments
///
/// * `ul` - Upper limit.
/// * `ll` - Lower limit.
/// * `std_dev` - Standard deviation.
///
/// # Example
///
/// ```rust
/// use dimstack_rs::stats::c_p;
/// let res = c_p(1.0, 0.0, 1.0);
/// assert_eq!(res, 0.16666666666666666);
/// ```
/// ```rust
/// use dimstack_rs::stats::c_p;
/// let res = c_p(6.0, -6.0, 1.0);
/// assert_eq!(res, 2.0);
/// ```
///
pub fn c_p(ul: f32, ll: f32, std_dev: f32) -> f32 {
    (ul - ll) / (6.0 * std_dev)
}

/// Process capability index. adjusted for centering.
///     
/// Cpl = (mu - L)/3*std_dev
/// Cpu = (U - mu)/3*std_dev
/// C_pk = min(Cpl, Cpu) = (1 - k) * C_p
///
/// # Arguments
///
/// * `ul` - Upper limit.
/// * `ll` - Lower limit.
/// * `mean` - Mean.
/// * `std_dev` - Standard deviation.
///
/// # Example
///
/// ```rust
/// use dimstack_rs::stats::c_pk;
/// let res = c_pk(208.036, 207.964, 208.009, 0.006);
/// assert_eq!((res * 100.0).round() / 100.0, 1.5); // rounded to 2 decimal places
/// ```
///
pub fn c_pk(ul: f32, ll: f32, mean: f32, std_dev: f32) -> f32 {
    f32::min((ul - mean) / (3.0 * std_dev), (mean - ll) / (3.0 * std_dev))
}

/// Root sum square.
///
/// # Arguments
///
/// * `args` - List of arguments.
///
/// # Example
///
/// ```
/// use dimstack_rs::stats::rss;
/// let args = vec![1.0, 2.0, 3.0];
/// let result = rss(&args);
/// assert_eq!(result, 3.7416573867739413);
/// ```
///
pub fn rss(args: &Vec<f32>) -> f32 {
    let mut val = 0.0;
    for arg in args {
        val += arg * arg;
    }
    val.sqrt()
}

/// Correction factor used to calculate the modified RSS.
///
/// # Arguments
///
/// * `t_rss` - _type_
/// * `t_wc` - _type_
/// * `n` - _type_
///
/// # Example
///
/// TODO
pub fn c_f(t_rss: f32, t_wc: f32, n: f32) -> f32 {
    ((0.5 * (t_wc - t_rss)) / (t_rss * (n.sqrt() - 1.0))) + 1.0
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_c_f() {
        // assert_eq!(c_f(1.0, 2.0, 3.0), 1.0);
    }
}
