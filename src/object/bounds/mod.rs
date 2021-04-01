use crate::Point3D;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct BoundingBox {
    pub min: Point3D,
    pub max: Point3D,
}

impl BoundingBox {
    // TODO temporary
    pub fn infinite() -> Self {
        BoundingBox {
            min: Point3D::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY),
            max: Point3D::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
        }
    }

    pub fn new(min: Point3D, max: Point3D) -> Self {
        assert!(
            min.x() <= max.x() && min.y() <= max.y() && min.z() <= max.z(),
            "Bounding box not correctly aligned"
        );
        BoundingBox { min, max }
    }

    pub fn expand_to_fit(&self, other: &Self) -> Self {
        let min = |axis: fn(Point3D) -> f64| axis(self.min).min(axis(other.min));
        let max = |axis: fn(Point3D) -> f64| axis(self.max).max(axis(other.max));

        BoundingBox {
            min: Point3D::new(min(|p| p.x()), min(|p| p.y()), min(|p| p.z())),
            max: Point3D::new(max(|p| p.x()), max(|p| p.y()), max(|p| p.z())),
        }
    }

    pub fn contains(&self, point: Point3D) -> bool {
        self.min.x() <= point.x()
            && self.min.y() <= point.y()
            && self.min.z() <= point.z()
            && self.max.x() >= point.x()
            && self.max.y() >= point.y()
            && self.max.z() >= point.z()
    }

    pub fn excludes(&self, point: Point3D) -> bool {
        !self.contains(point)
    }

    pub fn fully_contains(&self, other: &BoundingBox) -> bool {
        self.contains(other.min) && self.contains(other.max)
    }

    pub fn partially_excludes(&self, other: &BoundingBox) -> bool {
        !self.fully_contains(other)
    }
}

#[cfg(test)]
mod test_utils {
    use crate::object::bounds::BoundingBox;
    use crate::Point3D;
    use proptest::arbitrary::Arbitrary;
    use proptest::num;
    use proptest::prelude::{BoxedStrategy, Strategy};

    impl Arbitrary for BoundingBox {
        type Parameters = ();

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (
                num::f64::NEGATIVE,
                num::f64::NEGATIVE,
                num::f64::NEGATIVE,
                num::f64::POSITIVE,
                num::f64::POSITIVE,
                num::f64::POSITIVE,
            )
                .prop_map(|(x1, y1, z1, x2, y2, z2)| {
                    BoundingBox::new(Point3D::new(x1, y1, z1), Point3D::new(x2, y2, z2))
                })
                .boxed()
        }

        type Strategy = BoxedStrategy<Self>;
    }
}
