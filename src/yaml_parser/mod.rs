use crate::{Colour, Light, Point3D, Vector3D};
use either::Either::{Left, Right};
use yaml_rust::{Yaml, YamlLoader};

#[cfg(test)]
mod tests;

mod model;
use model::*;

//! Given the YAML example provided doesn't come with a schema, and doesn't provide many hints for
//! a parser implementation to use (e.g. type tags, or any consistent structure), the implementation
//! of the parser loosely resembles an event-driven streaming parser, as there's no way to navigate
//! the structure without resorting to iterating over the values in the outer array, and then making
//! educated guesses at the structure by looking for the presence of known keys with known values.
//!
//! Given the above limitations, the parser may behave very strangely when given unexpected input.
//!
//! From the single example and some common sense, I've reverse-engineered something resembling a schema,
//! but with a number of limitations:
//!
//! - The main structure is a list of objects
//! - The list must contain an object with an `add: camera` field (because it makes no sense to render a scene with no camera)
//! - The camera must contain a `width`, `height`, `field-of-view`, `from`, `to`, and `up` - there's not really sensible defaults for these
//! - 3-tuples i.e. `Colour`, `Point3D`, and `Vector3D` are described as arrays of numbers (Reals or Integers)
//! - Point lights are added using `add: light`
//! - Point lights must contain an `at` (position) and `intensity` (colour)
//! - Common definitions may be provided using a `define` object (an object that has `define` key)
//! - The value of the `define` key identifies it, and may be referenced by other defines or objects
//! - A `define` object with a `value` which is an object describes a material
//! - A `define` object with a `value` which is an array describes a sequence of transforms
//! - A material `define` may `extend` another material define
//! - A transform `define` may _reference_ another transform, but cannot `extend` one
//! - Materials may have any of the following fields: `color`, `diffuse`, `ambient`, `specular`, `shininess`, `reflective`, `transparency`, and `refractive-index` (`refraction`)
//! - A material that `extend`s another material will inherit any values that are defined by the extended material, but not by the child material
//! - Any missing fields on a material will be set to default values (see crate::material::Material)
//! - An entry in a transform `define` value array may either be a string referencing another transform define, or an array describing a transform
//! - A transform array's first field describes the transform type, and must be either `translate`, `scale`, `rotate-x`, `rotate-y`, `rotate-z`, or `shearing`
//! - A `translate` array must contain exactly 4 fields: `translate`, the `x` value, `y` value, and `z` value
//! - A `scale` array must contain exactly 4 fields: `scale`, the `x` value, `y` value, and `z` value
//! - A `rotate-x` array must contain exactly 2 fields: `rotate-x`, and the angle in radians
//! - A `rotate-y` array must contain exactly 2 fields: `rotate-y`, and the angle in radians
//! - A `rotate-z` array must contain exactly 2 fields: `rotate-z`, and the angle in radians
//! - No `shearing` transform is used in the example - this name matches the name in the book, but may be wrong
//! - Following the example of the other transforms, a `shearing` array must contain exactly 7 fields:
//!     - `shearing`
//!     - `x` in proportion to `y`
//!     - `x` in proportion to `z`
//!     - `y` in proportion to `x`
//!     - `y` in proportion to `z`
//!     - `z` in proportion to `x`
//!     - `z` in proportion to `y`
//! - Note: the identity value for a shear is 0
//! - Note: a shear with a non-zero e.g. x to y and y to x is not invertible, and therefore cannot be used
//! - Transforms must be described in reverse order, e.g. to rotate then translate, the translation must be described _first_
//!     - this is because naive matrix multiplication effectively creates a matrix that applies the right operand, then the left
//!     - therefore to use another `define`d transform, it must either be the first element of the array, to apply the child transforms _then_ the `define`d transform; or the last element to apply the `define`d transform first
//! - Objects may be added to the scene using an `add` - the object added depends on the value of `add`
//! - Objects that may be added are `plane`, `cube`, `sphere`, `cylinder`, and `cone` - triangles are not supported
//! - An object must have a `material` and a `transform`
//! - An object material may be a string referencing a define, or a material definition as described above
//! - An object's transforms must be an array of transforms, as described above
//! - To effectively apply the identity matrix instead (i.e. no transform), use an empty array `[]`
pub fn parse(input: &str) -> Result<SceneDescription, String> {
    match YamlLoader::load_from_str(input) {
        Ok(yaml) => {
            let mut camera = None;
            let mut lights = vec![];
            let mut defines = vec![];
            let mut objects = vec![];

            if let Some(items) = yaml[0].as_vec() {
                for item in items {
                    match item["add"].as_str() {
                        Some("camera") => {
                            camera = Some(item.parse()?);
                            continue;
                        }
                        Some("light") => {
                            lights.push(item.parse()?);
                            continue;
                        }
                        Some(_) => {
                            objects.push(item.parse()?);
                            continue;
                        }
                        None => (),
                    }

                    if item["define"].as_str().is_some() {
                        defines.push(item.parse()?);
                        continue;
                    }
                }
            } else {
                return Err("Expected a list of directives".to_string());
            }

            let camera = camera.ok_or("No `add: camera` directive found".to_string())?;

            Ok(SceneDescription {
                camera,
                lights,
                defines,
                objects,
            })
        }
        Err(error) => Err(error.to_string()),
    }
}

trait YamlExt {
    fn parse<T: FromYaml>(&self) -> Result<T, String>;
}

impl YamlExt for Yaml {
    fn parse<T: FromYaml>(&self) -> Result<T, String> {
        T::from_yaml(&self)
    }
}

trait FromYaml: Sized {
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
                let extends = yaml["extend"].as_str().map(Into::into);
                let material = value_node.parse()?;

                Ok(Define::Material {
                    name,
                    extends,
                    value: material,
                })
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
            _ => {
                return Err(format!(
                    "cannot parse `{}` as Object (note: only primitives are supported)",
                    add
                ))
            }
        };

        let material = match &yaml["material"] {
            Yaml::String(reference) => Left(reference.to_owned()),
            description @ Yaml::Hash(_) => Right(description.parse()?),
            other => return Err(format!("cannot parse object material; expected an Object describing the material, or a String referencing a defined material, at {:?}", other))
        };

        let transform = yaml["transform"].parse()?;

        Ok(ObjectDescription {
            kind,
            material,
            transform,
        })
    }
}

impl FromYaml for Transform {
    fn from_yaml(yaml: &Yaml) -> Result<Self, String> {
        use Transform::*;

        let transform = match yaml {
            Yaml::Array(transform) => transform,
            Yaml::String(reference) => {
                return Ok(Transform::Reference(reference.clone()))
            },
            _ => return Err(format!("Expected an Array describing a transform, or a String referencing a Define, at {:?}", yaml))
        };

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
        let colour = yaml["color"].parse()?;
        let diffuse = yaml["diffuse"].parse()?;
        let ambient = yaml["ambient"].parse()?;
        let specular = yaml["specular"].parse()?;
        let shininess = yaml["shininess"].parse()?;
        let reflective = yaml["reflective"].parse()?;
        let transparency = yaml["transparency"].parse()?;
        let refractive = yaml["refractive-index"].parse()?;

        Ok(MaterialDescription {
            colour,
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
