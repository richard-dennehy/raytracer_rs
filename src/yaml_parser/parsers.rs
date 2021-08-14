use crate::core::{Colour, Point3D, Vector3D};
use crate::scene::{CsgOperator, Light};
use crate::yaml_parser::model::{
    CameraDescription, MaterialDescription, ObjectDescription, ObjectKind, PatternKind,
    PatternType, Transformation, UvPatternType,
};
use crate::yaml_parser::model::{Define, Defines};
use std::num::{NonZeroU8, NonZeroUsize};
use yaml_rust::Yaml;

pub trait YamlExt {
    fn parse<T: FromYaml>(&self, defines: &Defines) -> Result<T, String>;
}

impl YamlExt for Yaml {
    fn parse<T: FromYaml>(&self, defines: &Defines) -> Result<T, String> {
        T::from_yaml_and_defines(&self, defines)
    }
}

pub trait FromYaml: Sized {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String>;
}

// FIXME ideally there'd be some kind of stack trace when parsing fails
impl FromYaml for f64 {
    fn from_yaml_and_defines(yaml: &Yaml, _: &Defines) -> Result<Self, String> {
        match &yaml {
            // yaml lib f64 parsing is lazy - this can't fail
            Yaml::Real(real) => Ok(real.parse().unwrap()),
            Yaml::Integer(integer) => Ok(*integer as f64),
            Yaml::BadValue => Err("value is undefined".into()),
            _ => Err(format!("cannot parse {:?} as floating point", yaml)),
        }
    }
}

impl FromYaml for usize {
    fn from_yaml_and_defines(yaml: &Yaml, _: &Defines) -> Result<Self, String> {
        match &yaml {
            Yaml::Integer(integer) if *integer >= 0 => Ok(*integer as usize),
            Yaml::Integer(_) => Err("value must not be negative".into()),
            Yaml::BadValue => Err("value is undefined".into()),
            _ => Err(format!("cannot parse {:?} as an integer", yaml)),
        }
    }
}

impl FromYaml for NonZeroU8 {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        yaml.parse::<usize>(defines).and_then(|int| {
            if int > u8::MAX as usize {
                Err(format!("value {:?} is too large", int))
            } else {
                NonZeroU8::new(int as u8).ok_or("value must not be 0".to_owned())
            }
        })
    }
}

impl FromYaml for NonZeroUsize {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        yaml.parse::<usize>(defines)
            .and_then(|int| NonZeroUsize::new(int).ok_or("value must not be 0".to_owned()))
    }
}

impl FromYaml for bool {
    fn from_yaml_and_defines(yaml: &Yaml, _: &Defines) -> Result<Self, String> {
        match &yaml {
            Yaml::Boolean(value) => Ok(*value),
            _ => Err(format!("cannot parse {:?} as a boolean", yaml)),
        }
    }
}

impl FromYaml for (f64, f64, f64) {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        if let Some(components) = yaml.as_vec() {
            if components.len() != 3 {
                return Err("Expected an array of exactly 3 numbers".into());
            } else {
                let x = components[0]
                    .parse(defines)
                    .map_err(|_| "cannot parse `x` component as floating point".to_string())?;
                let y = components[1]
                    .parse(defines)
                    .map_err(|_| "cannot parse `y` component as floating point".to_string())?;
                let z = components[2]
                    .parse(defines)
                    .map_err(|_| "cannot parse `z` component as floating point".to_string())?;

                Ok((x, y, z))
            }
        } else {
            Err("Expected an array of exactly 3 numbers".into())
        }
    }
}

impl FromYaml for ObjectDescription {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        let add = yaml["add"]
            .as_str()
            .or_else(|| yaml["type"].as_str())
            .ok_or_else(|| {
                "cannot parse YAML as ObjectDescription as it does not specify an `add` or a `type`"
            })?;

        let material = yaml["material"].parse(defines)?;
        let transforms = yaml["transform"].parse(defines)?;
        let casts_shadow = yaml["shadow"].parse::<Option<bool>>(defines)?;

        if let Some(define) = defines.get(add) {
            if let Define::Object(object) = define {
                Ok(object.extended(material, transforms, casts_shadow))
            } else {
                Err(format!("`define` {:?} is not an object", add))
            }
        } else {
            let kind = match add {
                "plane" => ObjectKind::Plane,
                "sphere" => ObjectKind::Sphere,
                "cube" => ObjectKind::Cube,
                "cylinder" => {
                    let min = yaml["min"].parse(defines)?;
                    let max = yaml["max"].parse(defines)?;

                    let capped = yaml["closed"].parse::<Option<_>>(defines)?.unwrap_or(false);

                    ObjectKind::Cylinder { min, max, capped }
                },
                "cone" => todo!("support cones"),
                "triangle" => return Err("adding triangles directly not supported - use an wavefront `obj` file to import meshes".into()),
                "obj" => {
                    let file_name = yaml["file"]
                        .as_str()
                        .ok_or_else(|| "must specify `file` name when adding an `obj`".to_string())?;
                    ObjectKind::ObjFile {
                        file_name: file_name.to_owned(),
                    }
                }
                "group" => {
                    ObjectKind::Group { children: yaml["children"].parse(defines)? }
                }
                "csg" => {
                    let operator = yaml["operation"].parse(defines)?;
                    let left = yaml["left"].parse(defines)?;
                    let right = yaml["right"].parse(defines)?;

                    ObjectKind::Csg { operator, left, right }
                }
                _ => return Err(format!("{:?} is not a primitive or a `define` (note: defines must be created before being referenced)", add)),
            };

            Ok(ObjectDescription {
                kind,
                material: material.unwrap_or_default(),
                transform: transforms.unwrap_or_default(),
                casts_shadow: casts_shadow.unwrap_or(true),
            })
        }
    }
}

impl FromYaml for CsgOperator {
    fn from_yaml_and_defines(yaml: &Yaml, _: &Defines) -> Result<Self, String> {
        match yaml.as_str() {
            Some("difference") => Ok(CsgOperator::Subtract),
            Some("union") => Ok(CsgOperator::Union),
            Some("intersection") => Ok(CsgOperator::Intersection),
            Some(other) => Err(format!("{:?} is not a valid CSG operation", other)),
            _ => Err(format!("cannot parse {:?} as a CSG operation", yaml)),
        }
    }
}

impl FromYaml for Vec<Transformation> {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        use Transformation::*;

        let mut transforms = Vec::new();

        let items = yaml
            .as_vec()
            .ok_or_else(|| "expected an array of transforms")?;

        for transform in items {
            match &transform {
                &Yaml::Array(transform) => {
                    let inline = match transform.get(0).and_then(Yaml::as_str) {
                        Some("translate") => {
                            assert_eq!(transform.len(), 4, "Expected translate to contain exactly 4 elements (including `translate`) at {:?}", transform);
                            let x = transform[1].parse(defines)?;
                            let y = transform[2].parse(defines)?;
                            let z = transform[3].parse(defines)?;
                            Translate { x, y, z }
                        }
                        Some("scale") => {
                            assert_eq!(
                                transform.len(),
                                4,
                                "Expected scale to contain exactly 4 elements (including `scale`) at {:?}",
                                transform
                            );
                            let x = transform[1].parse(defines)?;
                            let y = transform[2].parse(defines)?;
                            let z = transform[3].parse(defines)?;
                            Scale { x, y, z }
                        }
                        Some("rotate-x") => {
                            assert_eq!(
                                transform.len(),
                                2,
                                "Expected rotate to contain a single value, in radians"
                            );
                            let rotation = transform[1].parse(defines)?;
                            RotationX(rotation)
                        }
                        Some("rotate-y") => {
                            assert_eq!(
                                transform.len(),
                                2,
                                "Expected rotate to contain a single value, in radians"
                            );
                            let rotation = transform[1].parse(defines)?;
                            RotationY(rotation)
                        }
                        Some("rotate-z") => {
                            assert_eq!(
                                transform.len(),
                                2,
                                "Expected rotate to contain a single value, in radians"
                            );
                            let rotation = transform[1].parse(defines)?;
                            RotationZ(rotation)
                        }
                        Some("shear") => return Err("shear transforms are not supported".to_owned()),
                        Some(other) => return Err(format!("{:?} is not a type of transform (note: `define` references must be a string, not an array)", other)),
                        None => {
                            return Err(format!(
                                "Expected transform array first element to be a transformation name at {:?}",
                                transform
                            ))
                        }
                    };

                    transforms.push(inline)
                }
                &Yaml::String(reference) => {
                    if let Some(define) = defines.get(reference) {
                        if let Define::Transform(tfs) = define {
                            tfs.iter().for_each(|tf| transforms.push(tf.clone()))
                        }
                    }
                }
                _ => return Err(
                    "expected an array describing a transform, or a string referencing a `define`"
                        .into(),
                ),
            }
        }

        Ok(transforms)
    }
}

impl FromYaml for MaterialDescription {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        fn parse(yaml: &Yaml, defines: &Defines) -> Result<MaterialDescription, String> {
            let pattern = if yaml["color"].as_vec().is_some() {
                let colour = yaml["color"].parse(defines)?;
                Some(PatternKind::Solid(colour))
            } else {
                yaml["pattern"].parse::<Option<_>>(defines)?
            };
            let diffuse = yaml["diffuse"].parse(defines)?;
            let ambient = yaml["ambient"].parse(defines)?;
            let specular = yaml["specular"].parse(defines)?;
            let shininess = yaml["shininess"].parse(defines)?;
            let reflective = yaml["reflective"].parse(defines)?;
            let transparency = yaml["transparency"].parse(defines)?;
            let refractive = yaml["refractive-index"].parse(defines)?;

            Ok(MaterialDescription {
                pattern,
                diffuse,
                ambient,
                specular,
                shininess,
                reflective,
                transparency,
                refractive,
            })
        }

        // material is a simple reference to a define
        if let Some(reference) = yaml.as_str() {
            if let Some(define) = defines.get(reference) {
                if let Define::Material(material) = define {
                    Ok(material.clone())
                } else {
                    Err(format!("`define` {:?} is not a material", reference))
                }
            } else {
                Err(format!("`define` {:?} does not exist (note: a `define` must be created before it is referenced)", reference))
            }
        } else if yaml["value"].is_badvalue() {
            // material is defined inline
            parse(yaml, defines)
        } else {
            // material is a define (therefore fields are in a `value` node)
            let overrides = parse(&yaml["value"], defines)?;

            if let Some(extends) = yaml["extend"].as_str() {
                if let Some(define) = defines.get(extends) {
                    if let Define::Material(base) = define {
                        Ok(overrides.extend(base))
                    } else {
                        Err(format!("`define` {:?} is not a material", extends))
                    }
                } else {
                    Err(format!("`define` {:?} does not exist (note: a `define` must be created before it is referenced)", extends))
                }
            } else {
                Ok(overrides)
            }
        }
    }
}

impl FromYaml for CameraDescription {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        let width = yaml["width"].parse(defines)?;
        let height = yaml["height"].parse(defines)?;
        let field_of_view = yaml["field-of-view"].parse(defines)?;
        let from = yaml["from"].parse(defines)?;
        let to = yaml["to"].parse(defines)?;
        let up = yaml["up"].parse(defines)?;

        Ok(CameraDescription {
            width,
            height,
            field_of_view,
            from,
            to,
            up,
        })
    }
}

pub(in crate::yaml_parser) const DEFAULT_AREA_LIGHT_SEED: u64 = 4; // totally randomly chosen

impl FromYaml for Light {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        let colour = yaml["intensity"].parse(defines)?;

        // scene description format doesn't specify what kind of light to add, so have to guess based on what data is provided
        if let Some(position) = yaml["at"].parse(defines)? {
            Ok(Light::point(colour, position))
        } else {
            let bottom_left = yaml["corner"].parse(defines)?;
            let u = yaml["uvec"].parse(defines)?;
            let v = yaml["vvec"].parse(defines)?;
            let u_steps = yaml["usteps"].parse(defines)?;
            let v_steps = yaml["vsteps"].parse(defines)?;
            // ignore `jitter` - you don't get a choice between fixed vs random sampling
            Ok(Light::area(
                colour,
                bottom_left,
                u,
                v,
                u_steps,
                v_steps,
                DEFAULT_AREA_LIGHT_SEED,
            ))
        }
    }
}

impl FromYaml for PatternKind {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        let transforms = yaml["transform"].parse(defines)?;
        let colours = yaml["colors"]
            .parse::<Option<Vec<Colour>>>(defines)?
            .map(|colours| {
                if colours.len() != 2 {
                    Err("a pattern must have exactly 2 colours".to_string())
                } else {
                    Ok((colours[0], colours[1]))
                }
            });

        let pattern = match yaml["type"].as_str() {
            Some("stripes") => {
                let (primary, secondary) = colours
                    .ok_or("`stripes` pattern must specify exactly 2 colours".to_owned())??;

                PatternKind::Pattern {
                    pattern_type: PatternType::Stripes { primary, secondary },
                    transforms,
                }
            }
            Some("checkers") => {
                let (primary, secondary) = colours
                    .ok_or("`checkers` pattern must specify exactly 2 colours".to_owned())??;

                PatternKind::Pattern {
                    pattern_type: PatternType::Checkers { primary, secondary },
                    transforms,
                }
            }
            Some("rings") => {
                let (primary, secondary) = colours
                    .ok_or("`rings` pattern must specify exactly 2 colours".to_owned())??;

                PatternKind::Pattern {
                    pattern_type: PatternType::Rings { primary, secondary },
                    transforms,
                }
            }
            Some("map") => {
                let yaml = &yaml["uv_pattern"];
                match yaml["type"].as_str() {
                    Some("image") => {
                        let file_name = yaml["file"]
                            .as_str()
                            .ok_or("a UV image pattern must have a `file`".to_owned())?;
                        PatternKind::Uv {
                            uv_type: UvPatternType::Image {
                                file_name: file_name.to_owned(),
                            },
                            transforms,
                        }
                    }
                    Some("checkers") => {
                        let colours: Vec<Colour> = yaml["colors"].parse(defines)?;
                        if colours.len() != 2 {
                            return Err("a pattern must have exactly 2 colours".to_string());
                        }
                        let (primary, secondary) = (colours[0], colours[1]);
                        let width = yaml["width"].parse(defines)?;
                        let height = yaml["height"].parse(defines)?;

                        PatternKind::Uv {
                            uv_type: UvPatternType::Checkers {
                                primary,
                                secondary,
                                width,
                                height,
                            },
                            transforms,
                        }
                    }
                    Some(other) => {
                        return Err(format!("UV pattern type {} is not unsupported", other))
                    }
                    None => return Err(format!("A UV pattern must have a `type`")),
                }
            }
            Some(other) => return Err(format!("pattern type {} is not supported", other)),
            None => return Err("pattern must have a `type`".to_string()),
        };

        Ok(pattern)
    }
}

impl FromYaml for Define {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        // array of transforms or hash of material or hash of object
        match &yaml["value"] {
            array @ Yaml::Array(_) => Ok(Define::Transform(array.parse(defines)?)),
            hash @ Yaml::Hash(_) if hash["add"].as_str().is_some() => Ok(Define::Object(hash.parse(defines)?)),
            Yaml::Hash(_) => Ok(Define::Material(yaml.parse(defines)?)),
            _ => Err("expected `define` `value` to be an array of transforms, or a hash describing a material or an object".into())
        }
    }
}

// there's no way of implementing these generically without conflicting with Option, as that _also_
// defines From<(f64, f64, f64)> (or at least, From<T>)
impl FromYaml for Colour {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        yaml.parse(defines).map(|(r, g, b)| Self::new(r, g, b))
    }
}

impl FromYaml for Point3D {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        yaml.parse(defines).map(|(x, y, z)| Self::new(x, y, z))
    }
}

impl FromYaml for Vector3D {
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        yaml.parse(defines).map(|(x, y, z)| Self::new(x, y, z))
    }
}

impl<T> FromYaml for Option<T>
where
    T: FromYaml,
{
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        if yaml.is_badvalue() {
            Ok(None)
        } else {
            T::from_yaml_and_defines(&yaml, defines).map(Some)
        }
    }
}

impl<T> FromYaml for Vec<T>
where
    T: FromYaml,
{
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        match &yaml {
            Yaml::Array(array) => array
                .iter()
                .map(|item| T::from_yaml_and_defines(item, defines))
                .collect(),
            _ => Err(format!("expected array, got {:?}", yaml)),
        }
    }
}

impl<T> FromYaml for Box<T>
where
    T: FromYaml,
{
    fn from_yaml_and_defines(yaml: &Yaml, defines: &Defines) -> Result<Self, String> {
        T::from_yaml_and_defines(yaml, defines).map(Box::new)
    }
}
