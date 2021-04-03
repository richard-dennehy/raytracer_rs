use crate::{Point3D, Transform};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BoundingBox {
    pub min: Point3D,
    pub max: Point3D,
}

impl BoundingBox {
    // TODO temporary
    pub fn infinite() -> Self {
        BoundingBox {
            min: Point3D::new(-f64::MAX, -f64::MAX, -f64::MAX),
            max: Point3D::new(f64::MAX, f64::MAX, f64::MAX),
        }
    }

    pub fn new(min: Point3D, max: Point3D) -> Self {
        assert!(
            min.x() <= max.x() && min.y() <= max.y() && min.z() <= max.z(),
            "Bounding box not correctly aligned\n{:?} to {:?}",
            min,
            max
        );
        BoundingBox { min, max }
    }

    pub fn expand_to_fit(&self, other: &Self) -> Self {
        BoundingBox {
            min: Point3D::min([self.min, other.min]),
            max: Point3D::max([self.max, other.max]),
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

    pub fn transformed(&self, transformation: Transform) -> Self {
        // implementation is slightly complicated because a BoundingBox must be axis-aligned, and
        // naive rotation breaks that invariant
        let bottom_left_front = transformation * self.min;
        let bottom_left_back =
            transformation * Point3D::new(self.min.x(), self.min.y(), self.max.z());
        let bottom_right_back =
            transformation * Point3D::new(self.max.x(), self.min.y(), self.max.z());
        let bottom_right_front =
            transformation * Point3D::new(self.max.x(), self.min.y(), self.min.z());
        let top_right_front =
            transformation * Point3D::new(self.max.x(), self.max.y(), self.min.z());
        let top_left_front =
            transformation * Point3D::new(self.min.x(), self.max.y(), self.min.z());
        let top_left_back = transformation * Point3D::new(self.min.x(), self.max.y(), self.max.z());
        let top_right_back = transformation * self.max;

        let points = [
            bottom_left_front,
            bottom_left_back,
            bottom_right_back,
            bottom_right_front,
            top_right_front,
            top_left_front,
            top_left_back,
            top_right_back,
        ];

        BoundingBox::new(Point3D::min(points), Point3D::max(points))
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
