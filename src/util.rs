#[cfg(test)]
/// default f64 generator generates NaNs, enormous values, and minute values, all of which break
/// the calculations and test assertions, and none of which are reasonable input values
/// ("garbage in, garbage out" is a reasonable stance for a ray tracer)
/// this restricts f64s to a reasonable but still fairly generous range
#[derive(Clone, Debug, Copy)]
pub struct ReasonableF64(pub f64);

#[cfg(test)]
impl quickcheck::Arbitrary for ReasonableF64 {
    fn arbitrary(_: &mut quickcheck::Gen) -> Self {
        use rand::prelude::*;

        ReasonableF64(thread_rng().gen_range(-1000.0..1000.0))
    }
}
