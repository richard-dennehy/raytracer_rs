use crate::{Camera, Colour, Light, Object, Pattern, Point3D, Transform, Vector, Vector3D};
use either::Either;
use either::Either::{Left, Right};
use std::num::NonZeroU16;

#[derive(Debug, PartialEq)]
pub struct SceneDescription {
    pub(crate) camera: CameraDescription,
    pub(crate) lights: Vec<Light>,
    pub(crate) defines: Vec<Define>,
    pub(crate) objects: Vec<ObjectDescription>,
}

impl SceneDescription {
    pub fn override_resolution(&mut self, width: usize, height: usize) {
        self.camera.width = width;
        self.camera.height = height;
    }

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
        let transform = Transform::view_transform(
            self.camera.from,
            self.camera.to,
            self.camera.up.normalised(),
        );

        Ok(Camera::new(width, height, fov, transform))
    }

    pub fn lights(&self) -> Vec<Light> {
        self.lights.clone()
    }

    pub fn objects(&self) -> Result<Vec<Object>, String> {
        fn inner(
            this: &SceneDescription,
            objects: &Vec<ObjectDescription>,
        ) -> Result<Vec<Object>, String> {
            objects
                .iter()
                .map(|desc| {
                    let object = match &desc.kind {
                        ObjectKind::Plane => Ok(Object::plane()),
                        ObjectKind::Sphere => Ok(Object::sphere()),
                        ObjectKind::Cube => Ok(Object::cube()),
                        ObjectKind::Cylinder { min, max, capped } => {
                            let cylinder = Object::cylinder()
                                .min_y(min.unwrap_or(f64::INFINITY))
                                .max_y(max.unwrap_or(f64::INFINITY));

                            let cylinder = if *capped { cylinder.capped() } else { cylinder };

                            Ok(cylinder.build())
                        }
                        ObjectKind::ObjFile { .. } => todo!("load obj file"),
                        ObjectKind::Group { children } => inner(this, children).map(Object::group),
                        ObjectKind::Reference(..) => todo!("resolve obj reference")
                    };

                    let material_description = match &desc.material {
                        MaterialSource::Define(reference) => {
                            this.resolve_material(reference.as_str())
                        }
                        MaterialSource::Inline(desc) => Ok(desc.clone()),
                        MaterialSource::Undefined => Ok(MaterialDescription::default()),
                    };

                    let material = material_description.map(|d| d.to_material());
                    let transform = this
                        .to_transformations(&desc.transform)
                        .map(|tfs| tfs.to_matrix());

                    object.and_then(|obj| {
                        material.and_then(|mat| {
                            transform.map(|tf| obj.with_material(mat).transformed(tf))
                        })
                    })
                })
                .collect()
        }

        inner(self, &self.objects)
    }
    
    fn resolve_object(&self, name: &str) -> Result<ObjectDescription, String> {
        self.find(name).and_then(|def| match def {
            Define::Object {
                value,
                ..
            } => Ok(value.clone()),
            Define::MaterialDef { .. } => Err(format!("{:?} is a material, not an object", name)),
            Define::Transform { .. } => Err(format!("{:?} is a transform, not an object", name)),
        })
    }

    // FIXME circular `extend` will infinitely loop
    fn resolve_material(&self, name: &str) -> Result<MaterialDescription, String> {
        self.find(name)
            .and_then(|def| match def {
                Define::MaterialDef {
                    value,
                    extends: Some(extends),
                    ..
                } => {
                    let parent = self.resolve_material(extends);
                    parent.map(|p| MaterialDescription {
                        pattern: value.pattern.to_owned().or(p.pattern),
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
                Define::Object { .. } => {
                    Err(format!("{} is an object, not a material", def.name()))
                }
            })
    }

    fn to_transformations(&self, transforms: &Transforms) -> Result<Vec<Transformation>, String> {
        transforms
            .iter()
            .map(|tf| match tf {
                Left(define) => self
                    .resolve_transform(define)
                    .and_then(|tfs| self.to_transformations(tfs)),
                Right(tf) => Ok(vec![*tf]),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|nested| {
                nested
                    .into_iter()
                    .flat_map(|inner| inner.into_iter())
                    .collect()
            })
    }

    // FIXME can infinite loop
    fn resolve_transform(&self, name: &str) -> Result<&Transforms, String> {
        self.find(name)
            .and_then(|def| match def {
                Define::MaterialDef { .. } => {
                    Err(format!("{} is a material, not a transform", name))
                }
                Define::Object { .. } => {
                    Err(format!("{} is an object, not a transform", def.name()))
                }
                Define::Transform { value, .. } => Ok(value),
            })
    }

    fn find(&self, name: &str) -> Result<&Define, String> {
        self.defines.iter().find(|def| def.name() == name).ok_or_else(|| format!("reference has not been defined: {}", name))
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
// TODO might be easier to use Map<String, String> instead
pub enum Define {
    MaterialDef {
        name: String,
        extends: Option<String>,
        value: MaterialDescription,
    },
    Transform {
        name: String,
        value: Transforms,
    },
    Object {
        name: String,
        value: ObjectDescription,
    },
}

pub type Transforms = Vec<Either<String, Transformation>>;

impl Define {
    pub fn name(&self) -> &str {
        match &self {
            Define::MaterialDef { name, .. } => name.as_str(),
            Define::Transform { name, .. } => name.as_str(),
            Define::Object { name, .. } => name.as_str(),
        }
    }
}

#[derive(PartialEq, Debug, Default, Clone)]
pub struct MaterialDescription {
    pub(crate) pattern: Option<Either<Colour, PatternDescription>>,
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

        self.pattern
            .to_owned()
            .map(|pattern_desc| match pattern_desc {
                Left(colour) => material.pattern = Pattern::solid(colour),
                Right(pattern) => material.pattern = pattern.to_pattern(),
            });
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

#[derive(Debug, PartialEq, Clone)]
pub struct PatternDescription {
    pub(crate) pattern_type: PatternType,
    pub(crate) colours: (Colour, Colour),
    pub(crate) transforms: Option<Vec<Transformation>>,
}

impl PatternDescription {
    pub fn to_pattern(&self) -> Pattern {
        let (primary, secondary) = self.colours;

        let pattern = match self.pattern_type {
            PatternType::Stripes => Pattern::striped(primary, secondary),
            PatternType::Checker => Pattern::checkers(primary, secondary),
        };

        if let Some(tfs) = &self.transforms {
            pattern.with_transform(tfs.to_matrix())
        } else {
            pattern
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PatternType {
    Stripes, // TODO rest
    Checker,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Transformation {
    Translate { x: f64, y: f64, z: f64 },
    Scale { x: f64, y: f64, z: f64 },
    RotationX(f64),
    RotationY(f64),
    RotationZ(f64),
    // TODO shear
}

trait ToMatrix {
    fn to_matrix(&self) -> Transform;
}

impl ToMatrix for Vec<Transformation> {
    fn to_matrix(&self) -> Transform {
        self.iter()
            .map(|tf| match tf {
                Transformation::Translate { x, y, z } => Transform::identity()
                    .translate_x(*x)
                    .translate_y(*y)
                    .translate_z(*z),
                Transformation::Scale { x, y, z } => {
                    Transform::identity().scale_x(*x).scale_y(*y).scale_z(*z)
                }
                Transformation::RotationX(rads) => Transform::identity().rotate_x(*rads),
                Transformation::RotationY(rads) => Transform::identity().rotate_y(*rads),
                Transformation::RotationZ(rads) => Transform::identity().rotate_z(*rads),
            })
            .fold(Transform::identity(), |acc, next| next * acc)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ObjectDescription {
    pub kind: ObjectKind,
    pub material: MaterialSource,
    pub transform: Transforms,
    pub casts_shadow: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ObjectKind {
    Plane,
    Sphere,
    Cube,
    Cylinder {
        min: Option<f64>,
        max: Option<f64>,
        capped: bool,
    },
    ObjFile {
        file_name: String,
    },
    Group {
        children: Vec<ObjectDescription>,
    },
    Reference(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum MaterialSource {
    Define(String),
    Inline(MaterialDescription),
    Undefined,
}
