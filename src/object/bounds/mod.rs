use crate::{Point3D, Ray, Transform, Vector, Vector3D};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BoundingBox {
    pub min: Point3D,
    pub max: Point3D,
}

impl BoundingBox {
    // keep the maths vaguely functional - using INFINITY or f64::MAX breaks the calculations
    pub const LIMIT: f64 = f32::MAX as _;

    pub fn infinite() -> Self {
        BoundingBox {
            min: Point3D::new(-Self::LIMIT, -Self::LIMIT, -Self::LIMIT),
            max: Point3D::new(Self::LIMIT, Self::LIMIT, Self::LIMIT),
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

    #[allow(dead_code)]
    pub fn excludes(&self, point: Point3D) -> bool {
        !self.contains(point)
    }

    pub fn fully_contains(&self, other: &BoundingBox) -> bool {
        self.contains(other.min) && self.contains(other.max)
    }

    #[allow(dead_code)]
    pub fn partially_excludes(&self, other: &BoundingBox) -> bool {
        !self.fully_contains(other)
    }

    // TODO test plane (infinite) BBs don't break
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

    // see https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection
    pub fn intersected_by(&self, ray: &Ray) -> bool {
        fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
            let t_min_numerator = min - origin;
            let t_max_numerator = max - origin;

            let t_min = t_min_numerator / direction;
            let t_max = t_max_numerator / direction;

            if t_min > t_max {
                (t_max, t_min)
            } else {
                (t_min, t_max)
            }
        }

        let (t_min_x, t_max_x) = check_axis(
            ray.origin.x(),
            ray.direction.x(),
            self.min.x(),
            self.max.x(),
        );
        let (t_min_y, t_max_y) = check_axis(
            ray.origin.y(),
            ray.direction.y(),
            self.min.y(),
            self.max.y(),
        );

        if t_min_x > t_max_y || t_min_y > t_max_x {
            return false;
        }

        let (t_min_z, t_max_z) = check_axis(
            ray.origin.z(),
            ray.direction.z(),
            self.min.z(),
            self.max.z(),
        );

        let t_min = t_min_x.max(t_min_y).max(t_min_z);
        let t_max = t_max_x.min(t_max_y).min(t_max_z);

        t_max >= t_min
    }

    pub fn split(&self) -> (Self, Self) {
        let x_len = self.max.x() - self.min.x();
        let y_len = self.max.y() - self.min.y();
        let z_len = self.max.z() - self.min.z();

        if x_len >= y_len && x_len >= z_len {
            let halfway = self.max.x() - x_len / 2.0;

            let left =
                BoundingBox::new(self.min, Point3D::new(halfway, self.max.y(), self.max.z()));
            let right =
                BoundingBox::new(Point3D::new(halfway, self.min.y(), self.min.z()), self.max);

            (left, right)
        } else if y_len >= z_len {
            let halfway = self.max.y() - y_len / 2.0;

            let left =
                BoundingBox::new(self.min, Point3D::new(self.max.x(), halfway, self.max.z()));
            let right =
                BoundingBox::new(Point3D::new(self.min.x(), halfway, self.min.z()), self.max);

            (left, right)
        } else {
            let halfway = self.max.z() - z_len / 2.0;

            let left =
                BoundingBox::new(self.min, Point3D::new(self.max.x(), self.max.y(), halfway));
            let right =
                BoundingBox::new(Point3D::new(self.min.x(), self.min.y(), halfway), self.max);

            (left, right)
        }
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
