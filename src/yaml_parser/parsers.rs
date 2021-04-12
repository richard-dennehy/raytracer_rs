use crate::yaml_parser::model::{
    CameraDescription, Define, MaterialDescription, MaterialSource, ObjectDescription, ObjectKind,
    PatternDescription, PatternType, Transformation,
};
use crate::{Colour, Light, Point3D, Vector3D};
use either::Either;
use either::Either::{Left, Right};
use yaml_rust::Yaml;

pub trait YamlExt {
    fn parse<T: FromYaml>(&self) -> Result<T, String>;
}

impl YamlExt for Yaml {
    fn parse<T: FromYaml>(&self) -> Result<T, String> {
        T::from_yaml(&self)
    }
}

pub trait FromYaml: Sized {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String>;
}

impl FromYaml for f64 {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
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
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        match &yaml {
            Yaml::Integer(integer) if *integer >= 0 => Ok(*integer as usize),
            Yaml::Integer(_) => Err("value must not be negative".into()),
            Yaml::BadValue => Err("value is undefined".into()),
            _ => Err(format!("cannot parse {:?} as an integer", yaml)),
        }
    }
}

impl FromYaml for bool {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        match &yaml {
            Yaml::Boolean(value) => Ok(*value),
            _ => Err(format!("cannot parse {:?} as a boolean", yaml)),
        }
    }
}

impl FromYaml for (f64, f64, f64) {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        if let Some(components) = yaml.as_vec() {
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
}

impl FromYaml for Define {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        let name = yaml["define"]
            .as_str()
            .ok_or_else(|| "unreachable: define doesn't include a `define`".to_string())?
            .to_owned();

        let value_node = &yaml["value"];
        // this is awfully brittle, but the awkward format doesn't make this easy to parse
        match value_node {
            &Yaml::Hash(_) => {
                if value_node["add"].as_str().is_some() {
                    Ok(Define::Object {
                        name,
                        value: value_node.parse()?,
                    })
                } else {
                    let extends = yaml["extend"].as_str().map(Into::into);
                    let material = value_node.parse()?;

                    Ok(Define::MaterialDef {
                        name,
                        extends,
                        value: material,
                    })
                }
            }
            &Yaml::Array(_) => {
                let transform = value_node.parse()?;
                Ok(Define::Transform {
                    name,
                    value: transform,
                })
            }
            _ => Err(format!("cannot parse define at {}", name)),
        }
    }
}

impl FromYaml for ObjectDescription {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        let add = yaml["add"].as_str().ok_or_else(|| {
            "unreachable: should not be parsing yaml as ObjectDescription if there is no `add`"
                .to_string()
        })?;

        let kind = match add {
            "plane" => ObjectKind::Plane,
            "sphere" => ObjectKind::Sphere,
            "cube" => ObjectKind::Cube,
            "cylinder" => {
                let min = yaml["min"].parse()?;
                let max = yaml["max"].parse()?;

                let capped = yaml["closed"].parse::<Option<_>>()?.unwrap_or(false);

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
                ObjectKind::Group { children: yaml["children"].parse()? }
            }
            reference => ObjectKind::Reference(reference.to_owned()),
        };

        let material = match &yaml["material"] {
            Yaml::String(reference) => MaterialSource::Define(reference.to_owned()),
            description @ Yaml::Hash(_) => MaterialSource::Inline(description.parse()?),
            Yaml::BadValue => MaterialSource::Undefined,
            other => return Err(format!("cannot parse object material; expected an Object describing the material, or a String referencing a defined material, at {:?}", other))
        };

        let transform = yaml["transform"].parse::<Option<_>>()?.unwrap_or(vec![]);

        let casts_shadow = match &yaml["shadow"] {
            Yaml::Boolean(shadow) => *shadow,
            Yaml::BadValue => true,
            other => {
                return Err(format!(
                    "invalid `shadow` value; expected a boolean value, got {:?}",
                    other
                ))
            }
        };

        Ok(ObjectDescription {
            kind,
            material,
            transform,
            casts_shadow,
        })
    }
}

impl FromYaml for Either<String, Transformation> {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        match yaml {
            Yaml::Array(_) => yaml.parse().map(Right),
            Yaml::String(reference) => {
                Ok(Left(reference.clone()))
            },
            _ => Err(format!("Expected an Array describing a transform, or a String referencing a Define, at {:?}", yaml))
        }
    }
}

impl FromYaml for Transformation {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        use Transformation::*;

        let transform = yaml.as_vec().ok_or_else(|| "transforms must be an array")?;

        let parsed = match transform.get(0).and_then(Yaml::as_str) {
            Some("translate") => {
                assert_eq!(transform.len(), 4, "Expected translate to contain exactly 4 elements (including `translate`) at {:?}", transform);
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
                    transform
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
            Some(t) => todo!("transform {}", t),
            None => {
                return Err(format!(
                    "Expected transform array first element to be a transformation name at {:?}",
                    transform
                ))
            }
        };

        Ok(parsed)
    }
}

impl FromYaml for MaterialDescription {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        let pattern = if yaml["color"].as_vec().is_some() {
            let colour = yaml["color"].parse()?;
            Some(Left(colour))
        } else {
            yaml["pattern"].parse::<Option<_>>()?.map(Right)
        };
        let diffuse = yaml["diffuse"].parse()?;
        let ambient = yaml["ambient"].parse()?;
        let specular = yaml["specular"].parse()?;
        let shininess = yaml["shininess"].parse()?;
        let reflective = yaml["reflective"].parse()?;
        let transparency = yaml["transparency"].parse()?;
        let refractive = yaml["refractive-index"].parse()?;

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
}

impl FromYaml for CameraDescription {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        let width = yaml["width"].parse()?;
        let height = yaml["height"].parse()?;
        let field_of_view = yaml["field-of-view"].parse()?;
        let from = yaml["from"].parse()?;
        let to = yaml["to"].parse()?;
        let up = yaml["up"].parse()?;

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

impl FromYaml for Light {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        let colour = yaml["intensity"].parse()?;
        let position = yaml["at"].parse()?;

        Ok(Light::point(colour, position))
    }
}

impl FromYaml for PatternDescription {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        let pattern_type = match yaml["type"].as_str() {
            Some("stripes") => PatternType::Stripes,
            Some("checkers") => PatternType::Checker,
            Some(other) => todo!("pattern {}", other),
            None => return Err("pattern must have a `type`".to_string()),
        };

        let colours: Vec<Colour> = yaml["colors"].parse()?;
        if colours.len() != 2 {
            return Err("a pattern must have exactly 2 colours".to_string());
        }
        let colours = (colours[0], colours[1]);
        let transforms = yaml["transform"].parse()?;

        Ok(PatternDescription {
            pattern_type,
            colours,
            transforms,
        })
    }
}

// there's no way of implementing these generically without conflicting with Option, as that _also_
// defines From<(f64, f64, f64)> (or at least, From<T>)
impl FromYaml for Colour {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        yaml.parse().map(|(r, g, b)| Self::new(r, g, b))
    }
}

impl FromYaml for Point3D {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        yaml.parse().map(|(x, y, z)| Self::new(x, y, z))
    }
}

impl FromYaml for Vector3D {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        yaml.parse().map(|(x, y, z)| Self::new(x, y, z))
    }
}

impl<T> FromYaml for Option<T>
where
    T: FromYaml,
{
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        if yaml.is_badvalue() {
            Ok(None)
        } else {
            T::from_yaml(&yaml).map(Some)
        }
    }
}

impl<T> FromYaml for Vec<T>
where
    T: FromYaml,
{
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        match &yaml {
            Yaml::Array(array) => array.iter().map(T::from_yaml).collect(),
            _ => Err(format!("expected array, got {:?}", yaml)),
        }
    }
}
