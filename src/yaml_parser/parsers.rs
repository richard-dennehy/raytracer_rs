use crate::core::{Colour, Point3D, Vector3D};
use crate::scene::{CsgOperator, Light};
use crate::yaml_parser::model::Define;
use crate::yaml_parser::model::{
    CameraDescription, MaterialDescription, ObjectDescription, ObjectKind, PatternKind,
    PatternType, Transformation, UvPatternType,
};
use crate::yaml_parser::ParseState;
use std::num::{NonZeroU8, NonZeroUsize};
use yaml_rust::Yaml;

pub(in crate::yaml_parser) trait FromYaml: Sized {
    // todo rename
    fn from_yaml(parser: &ParseState) -> Result<Self, String>;
    fn type_name() -> String;
}

impl FromYaml for f64 {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        match parser.yaml() {
            // yaml lib f64 parsing is lazy - this can't fail
            Yaml::Real(real) => Ok(real.parse().unwrap()),
            Yaml::Integer(integer) => Ok(*integer as f64),
            Yaml::BadValue => Err("value is undefined".into()),
            other => Err(format!("cannot parse {:?} as floating point", other)),
        }
    }

    fn type_name() -> String {
        "Floating point".to_string()
    }
}

impl FromYaml for usize {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        match parser.yaml() {
            Yaml::Integer(integer) if *integer >= 0 => Ok(*integer as usize),
            Yaml::Integer(_) => Err("value must not be negative".into()),
            Yaml::BadValue => Err("value is undefined".into()),
            other => Err(format!("cannot parse {:?} as an integer", other)),
        }
    }

    fn type_name() -> String {
        "Non-negative integer".to_string()
    }
}

impl FromYaml for NonZeroU8 {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        usize::from_yaml(parser).and_then(|int| {
            if int > u8::MAX as usize {
                Err(format!("value {:?} is too large", int))
            } else {
                NonZeroU8::new(int as u8).ok_or("value must not be 0".to_owned())
            }
        })
    }

    fn type_name() -> String {
        "Non-negative integer".to_string()
    }
}

impl FromYaml for NonZeroUsize {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        usize::from_yaml(parser)
            .and_then(|int| NonZeroUsize::new(int).ok_or("value must not be 0".to_owned()))
    }

    fn type_name() -> String {
        "Non-negative integer".to_string()
    }
}

impl FromYaml for bool {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        match parser.yaml() {
            Yaml::Boolean(value) => Ok(*value),
            other => Err(format!("cannot parse {:?} as a boolean", other)),
        }
    }

    fn type_name() -> String {
        "Boolean".to_string()
    }
}

impl FromYaml for (f64, f64, f64) {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        if let Some(components) = parser.as_vec() {
            if components.len() != 3 {
                return Err("Expected an array of exactly 3 numbers".into());
            } else {
                let x = components[0]
                    .parse()
                    .map_err(|_| "cannot parse `x` component as floating point".to_string())?;
                let y = components[1]
                    .parse()
                    .map_err(|_| "cannot parse `y` component as floating point".to_string())?;
                let z = components[2]
                    .parse()
                    .map_err(|_| "cannot parse `z` component as floating point".to_string())?;

                Ok((x, y, z))
            }
        } else {
            Err("Expected an array of exactly 3 numbers".into())
        }
    }

    fn type_name() -> String {
        "Floating point array".to_string()
    }
}

impl FromYaml for ObjectDescription {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        let add = parser
            .get("add")
            .as_str()
            .or_else(|| parser.get("type").as_str())
            .ok_or_else(|| "must specify an `add` or a `type`")?;

        let material = parser.get("material").parse()?;
        let transforms = parser.get("transform").parse()?;
        let casts_shadow = parser.get("shadow").parse::<Option<bool>>()?;

        if let Some(define) = parser.defines.get(add) {
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
                    let min = parser.get("min").parse()?;
                    let max = parser.get("max").parse()?;

                    let capped = parser.get("closed").parse::<Option<bool>>()?.unwrap_or(false);

                    ObjectKind::Cylinder { min, max, capped }
                },
                "cone" => {
                    let min = parser.get("min").parse()?;
                    let max = parser.get("max").parse()?;

                    let capped = parser.get("closed").parse::<Option<bool>>()?.unwrap_or(false);

                    ObjectKind::Cone { min, max, capped }
                },
                "triangle" => return Err("adding triangles directly not supported - use an wavefront `obj` file to import meshes".into()),
                "obj" => {
                    let file_name = parser.get("file")
                        .as_str()
                        .ok_or_else(|| "must specify `file` name when adding an `obj`".to_string())?;
                    ObjectKind::ObjFile {
                        file_name: file_name.to_owned(),
                    }
                }
                "group" => {
                    ObjectKind::Group { children: parser.get("children").parse()? }
                }
                "csg" => {
                    let operator = parser.get("operation").parse()?;
                    let left = parser.get("left").parse()?;
                    let right = parser.get("right").parse()?;

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

    fn type_name() -> String {
        "Scene object".to_string()
    }
}

impl FromYaml for CsgOperator {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        match parser.yaml().as_str() {
            Some("difference") => Ok(CsgOperator::Subtract),
            Some("union") => Ok(CsgOperator::Union),
            Some("intersection") => Ok(CsgOperator::Intersection),
            Some(other) => Err(format!("{:?} is not a valid CSG operation", other)),
            _ => Err(format!(
                "cannot parse {:?} as a CSG operation",
                parser.yaml()
            )),
        }
    }

    fn type_name() -> String {
        "CSG operator".to_string()
    }
}

impl FromYaml for Vec<Transformation> {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        use Transformation::*;

        let mut transforms = Vec::new();

        let items = parser
            .as_vec()
            .ok_or_else(|| "expected an array of transforms")?;

        for item in items {
            match item.yaml() {
                Yaml::Array(_) => {
                    let transform = item.as_vec().unwrap();

                    let inline = match transform.get(0).and_then(ParseState::as_str) {
                        Some("translate") => {
                            assert_eq!(transform.len(), 4, "Expected translate to contain exactly 4 elements (including `translate`) at {:?}", item.yaml());
                            let x = transform[1].parse()?;
                            let y = transform[2].parse()?;
                            let z = transform[3].parse()?;
                            Translate { x, y, z }
                        }
                        Some("scale") => {
                            assert_eq!(
                                transform.len(),
                                4,
                                "Expected scale to contain exactly 4 elements (including `scale`) at {:?}",
                                item.yaml()
                            );
                            let x = transform[1].parse()?;
                            let y = transform[2].parse()?;
                            let z = transform[3].parse()?;
                            Scale { x, y, z }
                        }
                        Some("rotate-x") => {
                            assert_eq!(
                                transform.len(),
                                2,
                                "Expected rotate to contain a single value, in radians"
                            );
                            let rotation = transform[1].parse()?;
                            RotationX(rotation)
                        }
                        Some("rotate-y") => {
                            assert_eq!(
                                transform.len(),
                                2,
                                "Expected rotate to contain a single value, in radians"
                            );
                            let rotation = transform[1].parse()?;
                            RotationY(rotation)
                        }
                        Some("rotate-z") => {
                            assert_eq!(
                                transform.len(),
                                2,
                                "Expected rotate to contain a single value, in radians"
                            );
                            let rotation = transform[1].parse()?;
                            RotationZ(rotation)
                        }
                        Some("shear") => return Err("shear transforms are not supported".to_owned()),
                        Some(other) => return Err(format!("{:?} is not a type of transform (note: `define` references must be a string, not an array)", other)),
                        None => {
                            return Err(format!(
                                "Expected transform array first element to be a transformation name at {:?}",
                                item.yaml()
                            ))
                        }
                    };

                    transforms.push(inline)
                }
                Yaml::String(reference) => {
                    if let Some(define) = parser.defines.get(reference) {
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

    fn type_name() -> String {
        "Array of transforms".to_string()
    }
}

impl FromYaml for MaterialDescription {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        fn parse(parser: &ParseState) -> Result<MaterialDescription, String> {
            let pattern = if parser.get("color").as_vec().is_some() {
                let colour = parser.get("color").parse()?;
                Some(PatternKind::Solid(colour))
            } else {
                parser.get("pattern").parse::<Option<_>>()?
            };
            let diffuse = parser.get("diffuse").parse()?;
            let ambient = parser.get("ambient").parse()?;
            let specular = parser.get("specular").parse()?;
            let shininess = parser.get("shininess").parse()?;
            let reflective = parser.get("reflective").parse()?;
            let transparency = parser.get("transparency").parse()?;
            let refractive = parser.get("refractive-index").parse()?;

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
        if let Some(reference) = parser.yaml().as_str() {
            if let Some(define) = parser.defines.get(reference) {
                if let Define::Material(material) = define {
                    Ok(material.clone())
                } else {
                    Err(format!("`define` {:?} is not a material", reference))
                }
            } else {
                Err(format!("`define` {:?} does not exist (note: a `define` must be created before it is referenced)", reference))
            }
        } else if parser.get("value").yaml().is_badvalue() {
            // material is defined inline
            parse(parser)
        } else {
            // material is a define (therefore fields are in a `value` node)
            let overrides = parse(&parser.get("value"))?;

            if let Some(extends) = parser.get("extend").as_str() {
                if let Some(define) = parser.defines.get(extends) {
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

    fn type_name() -> String {
        "Material".to_string()
    }
}

impl FromYaml for CameraDescription {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        let width = parser.get("width").parse()?;
        let height = parser.get("height").parse()?;
        let field_of_view = parser.get("field-of-view").parse()?;
        let from = parser.get("from").parse()?;
        let to = parser.get("to").parse()?;
        let up = parser.get("up").parse()?;

        Ok(CameraDescription {
            width,
            height,
            field_of_view,
            from,
            to,
            up,
        })
    }

    fn type_name() -> String {
        "Camera".to_string()
    }
}

pub(in crate::yaml_parser) const DEFAULT_AREA_LIGHT_SEED: u64 = 4; // totally randomly chosen

impl FromYaml for Light {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        let colour = parser.get("intensity").parse()?;

        // scene description format doesn't specify what kind of light to add, so have to guess based on what data is provided
        if let Some(position) = parser.get("at").parse()? {
            Ok(Light::point(colour, position))
        } else {
            let bottom_left = parser.get("corner").parse()?;
            let u = parser.get("uvec").parse()?;
            let v = parser.get("vvec").parse()?;
            let u_steps = parser.get("usteps").parse()?;
            let v_steps = parser.get("vsteps").parse()?;
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

    fn type_name() -> String {
        "Light".to_string()
    }
}

impl FromYaml for PatternKind {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        let transforms = parser.get("transform").parse()?;

        let pattern_type = match parser.get("type").as_str() {
            Some("map") => {
                // awkwardness caused by awkward format
                return parser
                    .parse::<UvPatternType>()
                    .map(|uv_type| PatternKind::Uv {
                        uv_type,
                        transforms,
                    });
            }
            Some("stripes") => PatternType::Stripes,
            Some("checkers") => PatternType::Checkers,
            Some("rings") => PatternType::Rings,
            Some("gradient") => PatternType::Gradient,
            Some(other) => return Err(format!("pattern type {} is not supported", other)),
            None => return Err("pattern must have a `type`".to_string()),
        };

        let colours = parser
            .get("colors")
            .parse::<Vec<Colour>>()
            .and_then(|colours| {
                if colours.len() != 2 {
                    Err("a pattern must have exactly 2 colours".to_string())
                } else {
                    Ok((colours[0], colours[1]))
                }
            })?;

        Ok(PatternKind::Pattern {
            pattern_type,
            colours,
            transforms,
        })
    }

    fn type_name() -> String {
        "Pattern".to_string()
    }
}

impl FromYaml for UvPatternType {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        match parser.get("mapping").as_str() {
            Some("cube") => {
                return Ok(UvPatternType::Cube {
                    front: parser.get("front").parse()?,
                    back: parser.get("back").parse()?,
                    top: parser.get("up").parse()?,
                    bottom: parser.get("down").parse()?,
                    left: parser.get("left").parse()?,
                    right: parser.get("right").parse()?,
                })
            }
            Some("cylindrical") => {
                let sides = parser.get("uv_pattern").parse()?;

                let top = parser.get("top").parse::<Option<_>>()?;
                let bottom = parser.get("bottom").parse::<Option<_>>()?;

                let caps = match (top, bottom) {
                    (None, None) => None,
                    (Some(top), Some(bottom)) => Some((Box::new(top), Box::new(bottom))),
                    (Some(_), None) => return Err(
                        "a cylindrical map with a `top` pattern must also have a `bottom` pattern"
                            .to_owned(),
                    ),
                    (_, Some(_)) => return Err(
                        "a cylindrical map with a `bottom` pattern must also have a `top` pattern"
                            .to_owned(),
                    ),
                };

                return Ok(UvPatternType::Cylindrical { sides, caps });
            }
            Some("planar" | "spherical") => return parser.get("uv_pattern").parse(),
            Some(other) => return Err(format!("Unsupported UV mapping type {}", other)),
            _ => (), // recursive call
        }

        match parser.get("type").as_str() {
            Some("image") => {
                let file_name = parser
                    .get("file")
                    .as_str()
                    .ok_or("a UV image pattern must have a `file`".to_owned())?;

                Ok(UvPatternType::Image {
                    file_name: file_name.to_owned(),
                })
            }
            Some("checkers") => {
                let colours: Vec<Colour> = parser.get("colors").parse()?;
                if colours.len() != 2 {
                    return Err("a pattern must have exactly 2 colours".to_string());
                }
                let (primary, secondary) = (colours[0], colours[1]);
                let width = parser.get("width").parse()?;
                let height = parser.get("height").parse()?;

                Ok(UvPatternType::Checkers {
                    primary,
                    secondary,
                    width,
                    height,
                })
            }
            Some(other) => Err(format!("UV pattern type {} is not unsupported", other)),
            None => Err(format!("A UV pattern must have a `type`")),
        }
    }

    fn type_name() -> String {
        "UV pattern".to_string()
    }
}

impl FromYaml for Define {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        // array of transforms or hash of material or hash of object
        let value = parser.get("value");
        match value.yaml() {
            Yaml::Array(_) => Ok(Define::Transform(value.parse()?)),
            hash @ Yaml::Hash(_) if hash["add"].as_str().is_some() => {
                let context = hash["add"].as_str().unwrap().to_owned();
                Ok(Define::Object(value.with_extra_context(context).parse()?))
            },
            Yaml::Hash(_) => Ok(Define::Material(parser.parse()?)),
            _ => Err("expected `define` `value` to be an array of transforms, or a hash describing a material or an object".into())
        }
    }

    fn type_name() -> String {
        "Define".to_string()
    }
}

// there's no way of implementing these generically without conflicting with Option, as that _also_
// defines From<(f64, f64, f64)> (or at least, From<T>)
impl FromYaml for Colour {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        parser.parse().map(|(r, g, b)| Self::new(r, g, b))
    }

    fn type_name() -> String {
        "Colour".to_string()
    }
}

impl FromYaml for Point3D {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        parser.parse().map(|(x, y, z)| Self::new(x, y, z))
    }

    fn type_name() -> String {
        "Point".to_string()
    }
}

impl FromYaml for Vector3D {
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        parser.parse().map(|(x, y, z)| Self::new(x, y, z))
    }

    fn type_name() -> String {
        "Vector".to_string()
    }
}

impl<T> FromYaml for Option<T>
where
    T: FromYaml,
{
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        if parser.yaml().is_badvalue() {
            Ok(None)
        } else {
            T::from_yaml(parser).map(Some)
        }
    }

    fn type_name() -> String {
        format!("{} (Optional)", T::type_name())
    }
}

impl<T> FromYaml for Vec<T>
where
    T: FromYaml,
{
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        parser
            .as_vec()
            .map(|v| v.iter().map(T::from_yaml).collect())
            .unwrap_or(Err(format!("expected array, got {:?}", parser.yaml())))
    }

    fn type_name() -> String {
        format!("Array of {}", T::type_name())
    }
}

impl<T> FromYaml for Box<T>
where
    T: FromYaml,
{
    fn from_yaml(parser: &ParseState) -> Result<Self, String> {
        T::from_yaml(parser).map(Box::new)
    }

    fn type_name() -> String {
        T::type_name()
    }
}
