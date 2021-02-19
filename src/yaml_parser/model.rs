use crate::{Camera, Colour, Light, Matrix4D, Object, Pattern, Point3D, Vector3D};
use either::Either;
use std::num::NonZeroU16;

#[derive(Debug, PartialEq)]
pub struct SceneDescription {
    pub(crate) camera: CameraDescription,
    pub(crate) lights: Vec<Light>,
    pub(crate) defines: Vec<Define>,
    pub(crate) objects: Vec<ObjectDescription>,
}

impl SceneDescription {
    pub fn camera(&self) -> Result<Camera, String> {
        fn validate_nonzero_u16(dimension: &str, value: usize) -> Result<NonZeroU16, String> {
            let value = if value > (u16::MAX as usize) {
                return Err(format!("Camera {} is too large: {}", dimension, value));
            } else {
                value as u16
            };

            NonZeroU16::new(value).ok_or_else(|| format!("Camera {} cannot be zero", dimension))
        }

        let width = validate_nonzero_u16("width", self.camera.width)?;
        let height = validate_nonzero_u16("height", self.camera.height)?;
        let fov = self.camera.field_of_view;
        let transform = Matrix4D::view_transform(self.camera.from, self.camera.to, self.camera.up);

        Ok(Camera::new(width, height, fov, transform))
    }

    pub fn lights(&self) -> Vec<Light> {
        self.lights.clone()
    }

    pub fn objects(&self) -> Result<Vec<Object>, String> {
        self.objects
            .iter()
            .map(|desc| {
                let object = match desc.kind {
                    ObjectKind::Plane => Object::plane(),
                    ObjectKind::Sphere => Object::sphere(),
                    ObjectKind::Cube => Object::cube(),
                };

                let material_description = match &desc.material {
                    Either::Left(reference) => {
                        let define = self
                            .defines
                            .iter()
                            .find(|def| def.name() == reference.as_str())
                            .ok_or_else(|| {
                                format!("referenced material has not been defined: {}", reference)
                            });

                        define.and_then(|def| match def {
                            Define::Material { value, .. } => Ok(value),
                            Define::Transform { .. } => {
                                Err(format!("{} is a transform, not a material", def.name()))
                            }
                        })
                    }
                    Either::Right(desc) => Ok(desc),
                };

                let mut material = crate::Material::default();
                match material_description {
                    Ok(desc) => {
                        desc.colour
                            .map(|colour| material.pattern = Pattern::solid(colour));
                        desc.diffuse.map(|diffuse| material.diffuse = diffuse);
                        desc.ambient.map(|ambient| material.ambient = ambient);
                        desc.specular.map(|specular| material.specular = specular);
                        desc.shininess
                            .map(|shininess| material.shininess = shininess);
                        desc.reflective
                            .map(|reflective| material.reflective = reflective);
                        desc.transparency
                            .map(|transparency| material.transparency = transparency);
                        desc.refractive
                            .map(|refractive| material.refractive = refractive);
                    }
                    Err(_) => (),
                }

                let transform = Matrix4D::identity();

                Ok(object.with_transform(transform).with_material(material))
            })
            .collect()
    }
}

#[derive(PartialEq, Debug)]
pub struct CameraDescription {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) field_of_view: f64,
    pub(crate) from: Point3D,
    pub(crate) to: Point3D,
    pub(crate) up: Vector3D,
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
    pub(crate) colour: Option<Colour>,
    pub(crate) diffuse: Option<f64>,
    pub(crate) ambient: Option<f64>,
    pub(crate) specular: Option<f64>,
    pub(crate) shininess: Option<f64>,
    pub(crate) reflective: Option<f64>,
    pub(crate) transparency: Option<f64>,
    pub(crate) refractive: Option<f64>,
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
