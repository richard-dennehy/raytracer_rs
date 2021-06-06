use crate::material::MaterialKind;
use crate::obj_parser::ObjData;
use crate::{
    obj_parser, Camera, Colour, Light, Material, Object, Pattern, Point3D, Transform, Vector,
    Vector3D,
};
use either::Either;
use either::Either::{Left, Right};
use std::collections::HashMap;
use std::fs;
use std::num::NonZeroU16;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct SceneDescription {
    pub(crate) camera: CameraDescription,
    pub(crate) lights: Vec<Light>,
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
            obj_cache: &mut HashMap<String, ObjData>,
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
                        ObjectKind::ObjFile { file_name } => get_or_load(obj_cache, file_name),
                        ObjectKind::Group { children } => {
                            inner(this, children, obj_cache).map(Object::group)
                        }
                    };

                    let material = desc.material.to_material(desc.casts_shadow);
                    let transform = desc.transform.to_matrix();

                    object.map(|obj| {
                        if material == Material::default() {
                            obj.transformed(transform)
                        } else {
                            obj.transformed(transform).with_material(material)
                        }
                    })
                })
                .collect()
        }

        inner(self, &self.objects, &mut HashMap::new())
    }
}

fn get_or_load(
    cache: &mut HashMap<String, ObjData>,
    obj_file_name: &str,
) -> Result<Object, String> {
    if let Some(obj_data) = cache.get(obj_file_name) {
        return obj_data.to_object();
    }

    let file_contents = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("scene_descriptions")
            .join("obj_files")
            .join(obj_file_name),
    )
    .map_err(|e| e.to_string())?;

    let obj_data = obj_parser::parse(&file_contents);
    cache
        .entry(obj_file_name.to_string())
        .or_insert(obj_data)
        .to_object()
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
    fn to_material(&self, casts_shadow: bool) -> crate::Material {
        let mut material = crate::Material::default();

        self.pattern
            .to_owned()
            .map(|pattern_desc| match pattern_desc {
                Left(colour) => material.kind = MaterialKind::Solid(colour),
                Right(pattern) => material.kind = MaterialKind::Pattern(pattern.to_pattern()),
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
}
