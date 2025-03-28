use num_traits::cast::FromPrimitive;
use num_traits::float::Float;

// https://stackoverflow.com/questions/28655362/how-does-one-round-a-floating-point-number-to-a-specified-number-of-digits
fn round<T: Float + FromPrimitive>(x: T, decimals: Option<u32>) -> T {
    let y = T::from_i32(10i32.pow(decimals.unwrap_or(2))).unwrap();
    (x * y).round() / y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        assert_eq!(round(1.2345, None), 1.23);
        assert_eq!(round(1.2345, Some(1)), 1.2);
        assert_eq!(round(1.2345, Some(0)), 1.0);
        assert_eq!(round(1.2345, Some(3)), 1.235);
        // assert_eq!(round(4.365f32, Some(2)), 4.37);
        assert_eq!(round(4.365f64, Some(2)), 4.37);
    }
}
