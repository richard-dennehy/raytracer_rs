use crate::core::{Colour, Point3D, Transform, Vector3D, VectorMaths};
use crate::renderer::Camera;
use crate::scene::{CsgOperator, Light};
use crate::scene::{Material, MaterialKind, Pattern};
use crate::scene::{Object, UvPattern};
use crate::wavefront_parser::WavefrontParser;
use either::Either;
use either::Either::{Left, Right};
use std::collections::HashMap;
use std::num::{NonZeroU16, NonZeroUsize};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct SceneDescription {
    pub(crate) camera: CameraDescription,
    pub(crate) lights: Vec<Light>,
    pub(crate) objects: Vec<ObjectDescription>,
    pub(crate) resource_dir: PathBuf,
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
            desc: &ObjectDescription,
            parser: &WavefrontParser,
        ) -> Result<Object, String> {
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
                ObjectKind::ObjFile { file_name } => parser.load(file_name),
                ObjectKind::Group { children } => children
                    .iter()
                    .map(|c| inner(this, c, parser))
                    .collect::<Result<Vec<_>, _>>()
                    .map(Object::group),
                ObjectKind::Csg {
                    operator: CsgOperator::Subtract,
                    left,
                    right,
                } => inner(this, left, parser)
                    .and_then(|l| inner(this, right, parser).map(|r| Object::csg_difference(l, r))),
                ObjectKind::Csg {
                    operator: CsgOperator::Union,
                    left,
                    right,
                } => inner(this, left, parser)
                    .and_then(|l| inner(this, right, parser).map(|r| Object::csg_union(l, r))),
                ObjectKind::Csg {
                    operator: CsgOperator::Intersection,
                    left,
                    right,
                } => inner(this, left, parser).and_then(|l| {
                    inner(this, right, parser).map(|r| Object::csg_intersection(l, r))
                }),
            };

            let material = desc
                .material
                .to_material(desc.casts_shadow, &this.resource_dir);
            let transform = desc.transform.to_matrix();

            object.map(|obj| {
                if material == Material::default() {
                    obj.transformed(transform)
                } else {
                    obj.transformed(transform).with_material(material)
                }
            })
        }

        let parser = WavefrontParser::new(self.resource_dir.clone());
        self.objects
            .iter()
            .map(|desc| inner(self, desc, &parser))
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

#[derive(Debug, PartialEq)]
pub enum Define {
    Material(MaterialDescription),
    Transform(Vec<Transformation>),
    Object(ObjectDescription),
}

pub type Defines = HashMap<String, Define>;

#[derive(PartialEq, Debug, Default, Clone)]
pub struct MaterialDescription {
    // TODO doesn't support UVs
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
    // FIXME passing PathBuf here really sucks
    fn to_material(&self, casts_shadow: bool, resource_dir: &PathBuf) -> Material {
        let mut material = Material::default();

        self.pattern
            .to_owned()
            .map(|pattern_desc| match pattern_desc {
                Left(colour) => material.kind = MaterialKind::Solid(colour),
                Right(pattern) => {
                    material.kind = match pattern.to_pattern(resource_dir) {
                        Right(uv) => MaterialKind::Uv(uv),
                        Left(pattern) => MaterialKind::Pattern(pattern),
                    }
                }
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
        material.casts_shadow = casts_shadow;

        material
    }

    pub(crate) fn extend(self, base: &Self) -> Self {
        Self {
            pattern: self.pattern.or_else(|| base.pattern.clone()),
            diffuse: self.diffuse.or_else(|| base.diffuse),
            ambient: self.ambient.or_else(|| base.ambient),
            specular: self.specular.or_else(|| base.specular),
            shininess: self.shininess.or_else(|| base.shininess),
            reflective: self.reflective.or_else(|| base.reflective),
            transparency: self.transparency.or_else(|| base.transparency),
            refractive: self.refractive.or_else(|| base.refractive),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatternDescription {
    pub(crate) pattern_type: PatternType,
    pub(crate) transforms: Option<Vec<Transformation>>,
}

impl PatternDescription {
    // FIXME passing PathBuf here kind of sucks
    pub fn to_pattern(&self, resource_dir: &PathBuf) -> Either<Pattern, UvPattern> {
        let pattern = match &self.pattern_type {
            PatternType::Stripes { primary, secondary } => {
                Left(Pattern::striped(*primary, *secondary))
            }
            PatternType::Checkers { primary, secondary } => {
                Left(Pattern::checkers(*primary, *secondary))
            }
            PatternType::Rings { primary, secondary } => Left(Pattern::ring(*primary, *secondary)),
            PatternType::Uv(UvPatternType::Checkers {
                primary,
                secondary,
                width,
                height,
            }) => Right(UvPattern::checkers(*primary, *secondary, *width, *height)),
            PatternType::Uv(UvPatternType::Image { file_name }) => {
                let file_path = resource_dir.join(file_name);
                let img = image::open(&file_path)
                    .expect(&format!("failed to load uv pattern from {:?}", file_path));
                Right(UvPattern::image(Arc::new(img.to_rgb8())))
            }
        };

        if let Some(tfs) = &self.transforms {
            match pattern {
                Right(uv) => Right(uv.with_transform(tfs.to_matrix())),
                Left(pattern) => Left(pattern.with_transform(tfs.to_matrix())),
            }
        } else {
            pattern
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternType {
    Stripes { primary: Colour, secondary: Colour },
    Checkers { primary: Colour, secondary: Colour },
    Rings { primary: Colour, secondary: Colour },
    // FIXME should probably just use `UvPatternType` directly
    Uv(UvPatternType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum UvPatternType {
    Checkers {
        primary: Colour,
        secondary: Colour,
        width: NonZeroUsize,
        height: NonZeroUsize,
    },
    Image {
        file_name: String,
    },
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Transformation {
    Translate { x: f64, y: f64, z: f64 },
    Scale { x: f64, y: f64, z: f64 },
    RotationX(f64),
    RotationY(f64),
    RotationZ(f64),
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
    pub material: MaterialDescription,
    pub transform: Vec<Transformation>,
    pub casts_shadow: bool,
}

impl ObjectDescription {
    pub(crate) fn extended(
        &self,
        material: Option<MaterialDescription>,
        transforms: Option<Vec<Transformation>>,
        casts_shadow: Option<bool>,
    ) -> Self {
        Self {
            kind: self.kind.clone(),
            material: match material {
                Some(material) => self.material.clone().extend(&material),
                None => self.material.clone(),
            },
            transform: match transforms {
                Some(transforms) => self
                    .transform
                    .iter()
                    .cloned()
                    .chain(transforms.into_iter())
                    .collect(),
                None => self.transform.clone(),
            },
            casts_shadow: casts_shadow.unwrap_or(self.casts_shadow),
        }
    }
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
    Csg {
        operator: CsgOperator,
        left: Box<ObjectDescription>,
        right: Box<ObjectDescription>,
    },
}
