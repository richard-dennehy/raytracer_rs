use crate::{Colour, Light, Point3D, Vector3D};
use either::Either;

#[derive(Debug, PartialEq)]
pub struct SceneDescription {
    pub camera: CameraDescription,
    pub lights: Vec<Light>,
    pub defines: Vec<Define>,
    pub objects: Vec<ObjectDescription>,
}

#[derive(PartialEq, Debug)]
pub struct CameraDescription {
    pub width: usize,
    pub height: usize,
    pub field_of_view: f64,
    pub from: Point3D,
    pub to: Point3D,
    pub up: Vector3D,
}

#[derive(PartialEq, Debug)]
pub enum Define {
    Material {
        name: String,
        extends: Option<String>,
        value: MaterialDescription,
    },
    Transform {
        name: String,
        value: Vec<Transform>,
    },
}

impl Define {
    pub fn name(&self) -> &str {
        match &self {
            Define::Material { name, .. } => name.as_str(),
            Define::Transform { name, .. } => name.as_str(),
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct MaterialDescription {
    pub colour: Option<Colour>,
    pub diffuse: Option<f64>,
    pub ambient: Option<f64>,
    pub specular: Option<f64>,
    pub shininess: Option<f64>,
    pub reflective: Option<f64>,
    pub transparency: Option<f64>,
    pub refractive: Option<f64>,
}

#[derive(PartialEq, Debug)]
pub enum Transform {
    Translate { x: f64, y: f64, z: f64 },
    Scale { x: f64, y: f64, z: f64 },
    RotationX(f64),
    RotationY(f64),
    RotationZ(f64),
    // for some reason, combining transforms is defined inline, rather than using `extend`, like materials
    Reference(String),
    // TODO shear
}

#[derive(PartialEq, Debug)]
pub struct ObjectDescription {
    pub kind: ObjectKind,
    pub material: Either<String, MaterialDescription>,
    pub transform: Vec<Transform>,
}

#[derive(PartialEq, Debug)]
pub enum ObjectKind {
    Plane,
    Sphere,
    Cube,
}
