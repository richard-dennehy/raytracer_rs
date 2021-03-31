use crate::Point3D;

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
}
