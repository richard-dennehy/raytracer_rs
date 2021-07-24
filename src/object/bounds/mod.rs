use crate::{Point3D, Ray, Transform, Vector};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BoundingBox {
    // bounds[0] == min; bounds[1] == max
    bounds: [Point3D; 2],
}

impl BoundingBox {
    // keep the maths vaguely functional - using INFINITY or f64::MAX breaks the calculations
    pub const LIMIT: f64 = f32::MAX as _;

    pub fn infinite() -> Self {
        BoundingBox {
            bounds: [
                Point3D::new(-Self::LIMIT, -Self::LIMIT, -Self::LIMIT),
                Point3D::new(Self::LIMIT, Self::LIMIT, Self::LIMIT),
            ],
        }
    }

    pub fn new(min: Point3D, max: Point3D) -> Self {
        // keep the maths vaguely sane
        let min = Point3D::new(
            min.x().max(-Self::LIMIT),
            min.y().max(-Self::LIMIT),
            min.z().max(-Self::LIMIT),
        );
        let max = Point3D::new(
            max.x().min(Self::LIMIT),
            max.y().min(Self::LIMIT),
            max.z().min(Self::LIMIT),
        );

        BoundingBox { bounds: [min, max] }
    }

    pub fn expand_to_fit(&self, other: &Self) -> Self {
        BoundingBox {
            bounds: [
                Point3D::min([self.bounds[0], other.bounds[0]]),
                Point3D::max([self.bounds[1], other.bounds[1]]),
            ],
        }
    }

    pub fn contains(&self, point: Point3D) -> bool {
        let min = self.bounds[0];
        let max = self.bounds[1];

        min.x() <= point.x()
            && min.y() <= point.y()
            && min.z() <= point.z()
            && max.x() >= point.x()
            && max.y() >= point.y()
            && max.z() >= point.z()
    }

    #[allow(dead_code)]
    pub fn excludes(&self, point: Point3D) -> bool {
        !self.contains(point)
    }

    pub fn fully_contains(&self, other: &BoundingBox) -> bool {
        self.contains(other.bounds[0]) && self.contains(other.bounds[1])
    }

    #[allow(dead_code)]
    pub fn partially_excludes(&self, other: &BoundingBox) -> bool {
        !self.fully_contains(other)
    }

    // TODO test plane (infinite) BBs don't break
    pub fn transformed(&self, transformation: Transform) -> Self {
        let min = self.bounds[0];
        let max = self.bounds[1];

        // implementation is slightly complicated because a BoundingBox must be axis-aligned, and
        // naive rotation breaks that invariant
        let bottom_left_front = transformation * min;
        let bottom_left_back = transformation * Point3D::new(min.x(), min.y(), max.z());
        let bottom_right_back = transformation * Point3D::new(max.x(), min.y(), max.z());
        let bottom_right_front = transformation * Point3D::new(max.x(), min.y(), min.z());
        let top_right_front = transformation * Point3D::new(max.x(), max.y(), min.z());
        let top_left_front = transformation * Point3D::new(min.x(), max.y(), min.z());
        let top_left_back = transformation * Point3D::new(min.x(), max.y(), max.z());
        let top_right_back = transformation * max;

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

    pub fn intersected_by(&self, ray: &Ray) -> bool {
        // roughly adapted from https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection
        // note that the "optimised" version shown at the link above doesn't appear to be significantly faster when translated into rust
        // (including using unsafe functions to bypass array bounds checking) and definitely isn't fast enough to justify the excessively terse code
        let t_min_x = (self.bounds[0].x() - ray.origin.x()) / ray.direction.x();
        let t_max_x = (self.bounds[1].x() - ray.origin.x()) / ray.direction.x();

        let (t_min_x, t_max_x) = if ray.direction.x().is_sign_negative() {
            (t_max_x, t_min_x)
        } else {
            (t_min_x, t_max_x)
        };

        let t_min_y = (self.bounds[0].y() - ray.origin.y()) / ray.direction.y();
        let t_max_y = (self.bounds[1].y() - ray.origin.y()) / ray.direction.y();

        let (t_min_y, t_max_y) = if ray.direction.y().is_sign_negative() {
            (t_max_y, t_min_y)
        } else {
            (t_min_y, t_max_y)
        };

        // bitwise or yields a significant performance improvement over short-circuiting or;
        // it possibly plays better with the branch predictor
        if (t_min_x > t_max_y) | (t_min_y > t_max_x) {
            return false;
        }

        let t_min = t_min_x.max(t_min_y);
        let t_max = t_max_x.min(t_max_y);

        let t_min_z = (self.bounds[0].z() - ray.origin.z()) / ray.direction.z();
        let t_max_z = (self.bounds[1].z() - ray.origin.z()) / ray.direction.z();

        let (t_min_z, t_max_z) = if ray.direction.z().is_sign_negative() {
            (t_max_z, t_min_z)
        } else {
            (t_min_z, t_max_z)
        };

        if (t_min > t_max_z) | (t_min_z > t_max) {
            return false;
        }

        return true;
    }

    pub fn split(&self) -> (Self, Self) {
        let min = self.bounds[0];
        let max = self.bounds[1];

        let x_len = max.x() - min.x();
        let y_len = max.y() - min.y();
        let z_len = max.z() - min.z();

        if x_len >= y_len && x_len >= z_len {
            let halfway = max.x() - x_len / 2.0;

            let left = BoundingBox::new(min, Point3D::new(halfway, max.y(), max.z()));
            let right = BoundingBox::new(Point3D::new(halfway, min.y(), min.z()), max);

            (left, right)
        } else if y_len >= z_len {
            let halfway = max.y() - y_len / 2.0;

            let left = BoundingBox::new(min, Point3D::new(max.x(), halfway, max.z()));
            let right = BoundingBox::new(Point3D::new(min.x(), halfway, min.z()), max);

            (left, right)
        } else {
            let halfway = max.z() - z_len / 2.0;

            let left = BoundingBox::new(min, Point3D::new(max.x(), max.y(), halfway));
            let right = BoundingBox::new(Point3D::new(min.x(), min.y(), halfway), max);

            (left, right)
        }
    }

    #[cfg(test)]
    pub fn min(&self) -> Point3D {
        self.bounds[0]
    }

    #[cfg(test)]
    pub fn max(&self) -> Point3D {
        self.bounds[1]
    }
}

#[cfg(test)]
mod test_utils {
    use crate::object::bounds::BoundingBox;
    use crate::Point3D;
    use quickcheck::{Arbitrary, Gen};
    use rand::prelude::*;

    impl Arbitrary for BoundingBox {
        fn arbitrary(_: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();

            fn gen_positive(rng: &mut ThreadRng) -> f64 {
                rng.gen_range(0.0..10.0)
            }

            fn gen_negative(rng: &mut ThreadRng) -> f64 {
                rng.gen_range(-10.0..0.0)
            }

            BoundingBox::new(
                Point3D::new(
                    gen_negative(&mut rng),
                    gen_negative(&mut rng),
                    gen_negative(&mut rng),
                ),
                Point3D::new(
                    gen_positive(&mut rng),
                    gen_positive(&mut rng),
                    gen_positive(&mut rng),
                ),
            )
        }
    }
}
