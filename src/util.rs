pub fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    };

    let first = (-b - discriminant.sqrt()) / (2.0 * a);
    let second = (-b + discriminant.sqrt()) / (2.0 * a);

    Some((first, second))
}

#[cfg(test)]
/// default f64 generator generates NaNs, enormous values, and minute values, all of which break
/// the calculations and test assertions, and none of which are reasonable input values
/// ("garbage in, garbage out" is a reasonable stance for a ray tracer)
/// this restricts f64s to a reasonable but still fairly generous range
pub fn reasonable_f64() -> std::ops::Range<f64> {
    -1000.0..1000.0
}
