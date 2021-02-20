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
                    Either::Left(reference) => self.resolve_material(reference.as_str()),
                    Either::Right(desc) => Ok(desc.clone()),
                };

                let material = material_description.map(|d| d.to_material());

                let transform = desc
                    .transform
                    .iter()
                    .map(|tf| match tf {
                        Transform::Translate { x, y, z } => Ok(Matrix4D::translation(*x, *y, *z)),
                        Transform::Scale { x, y, z } => Ok(Matrix4D::scaling(*x, *y, *z)),
                        Transform::RotationX(rads) => Ok(Matrix4D::rotation_x(*rads)),
                        Transform::RotationY(rads) => Ok(Matrix4D::rotation_y(*rads)),
                        Transform::RotationZ(rads) => Ok(Matrix4D::rotation_z(*rads)),
                        Transform::Reference(name) => self.resolve_transform(name.as_str()),
                    })
                    .fold(Ok(Matrix4D::identity()), |acc, next| {
                        acc.and_then(|lhs| next.map(|rhs| lhs * rhs))
                    });

                material.and_then(|mat| {
                    transform.map(|tf| object.with_material(mat).with_transform(tf))
                })
            })
            .collect()
    }

    fn resolve_material(&self, name: &str) -> Result<MaterialDescription, String> {
        self.defines
            .iter()
            .find(|def| def.name() == name)
            .ok_or_else(|| format!("referenced material has not been defined: {}", name))
            .and_then(|def| match def {
                Define::MaterialDef {
                    value,
                    extends: Some(extends),
                    ..
                } => {
                    let parent = self.resolve_material(extends);
                    parent.map(|p| MaterialDescription {
                        colour: value.colour.or(p.colour),
                        diffuse: value.diffuse.or(p.diffuse),
                        ambient: value.ambient.or(p.ambient),
                        specular: value.specular.or(p.specular),
                        shininess: value.shininess.or(p.shininess),
                        reflective: value.reflective.or(p.reflective),
                        transparency: value.transparency.or(p.transparency),
                        refractive: value.refractive.or(p.refractive),
                    })
                }
                Define::MaterialDef { value, .. } => Ok(value.clone()),
                Define::Transform { .. } => {
                    Err(format!("{} is a transform, not a material", def.name()))
                }
            })
    }

    fn resolve_transform(&self, name: &str) -> Result<Matrix4D, String> {
        self.defines
            .iter()
            .find(|def| def.name() == name)
            .ok_or_else(|| format!("referenced transform has not been defined: {}", name))
            .and_then(|def| match def {
                Define::MaterialDef { .. } => {
                    Err(format!("{} is a material, not a transform", name))
                }
                Define::Transform { value, .. } => Ok(value),
            })
            .and_then(|tfs| {
                tfs.iter()
                    .map(|tf| match tf {
                        Transform::Translate { x, y, z } => Ok(Matrix4D::translation(*x, *y, *z)),
                        Transform::Scale { x, y, z } => Ok(Matrix4D::scaling(*x, *y, *z)),
                        Transform::RotationX(rads) => Ok(Matrix4D::rotation_x(*rads)),
                        Transform::RotationY(rads) => Ok(Matrix4D::rotation_y(*rads)),
                        Transform::RotationZ(rads) => Ok(Matrix4D::rotation_z(*rads)),
                        Transform::Reference(name) => self.resolve_transform(name.as_str()),
                    })
                    .fold(Ok(Matrix4D::identity()), |acc, next| {
                        acc.and_then(|lhs| next.map(|rhs| lhs * rhs))
                    })
            })
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
    MaterialDef {
        name: String,
        extends: Option<String>,
        value: MaterialDescription,
    },
    Transform {
        name: String,
        // FIXME change this to Vec<Either<Transform, String>>
        value: Vec<Transform>,
    },
}

impl Define {
    pub fn name(&self) -> &str {
        match &self {
            Define::MaterialDef { name, .. } => name.as_str(),
            Define::Transform { name, .. } => name.as_str(),
        }
    }
}

#[derive(PartialEq, Debug, Default, Clone)]
pub struct MaterialDescription {
    // FIXME support pattern definitions
    pub(crate) colour: Option<Colour>,
    pub(crate) diffuse: Option<f64>,
    pub(crate) ambient: Option<f64>,
    pub(crate) specular: Option<f64>,
    pub(crate) shininess: Option<f64>,
    pub(crate) reflective: Option<f64>,
    pub(crate) transparency: Option<f64>,
    pub(crate) refractive: Option<f64>,
}

impl MaterialDescription {
    fn to_material(&self) -> crate::Material {
        let mut material = crate::Material::default();

        self.colour
            .map(|colour| material.pattern = Pattern::solid(colour));
        self.diffuse.map(|diffuse| material.diffuse = diffuse);
        self.ambient.map(|ambient| material.ambient = ambient);
        self.specular.map(|specular| material.specular = specular);
        self.shininess
            .map(|shininess| material.shininess = shininess);
        self.reflective
            .map(|reflective| material.reflective = reflective);
        self.transparency
            .map(|transparency| material.transparency = transparency);
        self.refractive
            .map(|refractive| material.refractive = refractive);

        material
    }
}

// TODO impl to_matrix for Vec<Transform>
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
