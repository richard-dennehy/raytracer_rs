use crate::{Light, Point3D};
use either::Either::{Left, Right};
use yaml_rust::{Yaml, YamlLoader};

#[cfg(test)]
mod tests;

mod model;
use model::*;

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
                            camera = Some(parse_camera(&item)?);
                            continue;
                        }
                        Some("light") => {
                            lights.push(parse_light(&item)?);
                            continue;
                        }
                        Some(object_type) => {
                            objects.push(parse_object(&item, object_type)?);
                            continue;
                        }
                        None => (),
                    }

                    if let Some(name) = item["define"].as_str() {
                        defines.push(parse_define(item, name)?);
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

fn parse_camera(yaml: &Yaml) -> Result<CameraDescription, String> {
    let width = parse_usize(yaml, "width")?;
    let height = parse_usize(yaml, "height")?;
    let field_of_view = parse_f64(yaml, "field-of-view")?;
    let from = parse_tuple(yaml, "from")?.into();
    let to = parse_tuple(yaml, "to")?.into();
    let up = parse_tuple(yaml, "up")?.into();

    Ok(CameraDescription {
        width,
        height,
        field_of_view,
        from,
        to,
        up,
    })
}

fn parse_light(yaml: &Yaml) -> Result<Light, String> {
    let colour = parse_tuple(&yaml, "intensity")?.into();
    let position = parse_tuple(&yaml, "at")?.into();

    Ok(Light::point(colour, position))
}

fn parse_define(yaml: &Yaml, name: &str) -> Result<Define, String> {
    let name = name.to_string();

    let value_node = &yaml["value"];
    // this is awfully brittle, but the awkward format doesn't make this easy to parse
    match value_node {
        &Yaml::Hash(_) => {
            let extends = yaml["extend"].as_str().map(Into::into);
            let material = parse_material(value_node)?;

            Ok(Define::Material {
                name,
                extends,
                value: material,
            })
        }
        &Yaml::Array(_) => {
            let transform = parse_transform(value_node)?;
            Ok(Define::Transform {
                name,
                value: transform,
            })
        }
        _ => Err(format!("cannot parse define at {}", name)),
    }
}

fn parse_material(yaml: &Yaml) -> Result<MaterialDescription, String> {
    // FIXME can't distinguish between missing optional properties and invalid values
    let colour = parse_tuple(&yaml, "color").ok().map(Into::into);
    let diffuse = to_f64_lenient(&yaml["diffuse"]).ok();
    let ambient = to_f64_lenient(&yaml["ambient"]).ok();
    let specular = to_f64_lenient(&yaml["specular"]).ok();
    let shininess = to_f64_lenient(&yaml["shininess"]).ok();
    let reflective = to_f64_lenient(&yaml["reflective"]).ok();
    let transparency = to_f64_lenient(&yaml["transparency"]).ok();
    let refractive = to_f64_lenient(&yaml["refractive-index"]).ok();

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

fn parse_transform(yaml: &Yaml) -> Result<Vec<Transform>, String> {
    let array = match &yaml {
        &Yaml::Array(array) => array,
        _ => unreachable!(),
    };

    let mut transforms = Vec::with_capacity(array.len());

    use Transform::*;

    for item in array.iter() {
        let transform = match item {
            Yaml::Array(transform) => transform,
            Yaml::String(reference) => {
                transforms.push(Transform::Reference(reference.clone()));
                continue
            },
            _ => return Err(format!("Expected an Array describing a transform, or a String referencing a Define, at {:?}", item))
        };

        let parsed = match transform.get(0).and_then(Yaml::as_str) {
            Some("translate") => {
                assert_eq!(transform.len(), 4, "Expected translate to contain exactly 4 elements (including `translate`) at {:?}", transform);
                let x = to_f64_lenient(&transform[1])?;
                let y = to_f64_lenient(&transform[2])?;
                let z = to_f64_lenient(&transform[3])?;
                Translate { x, y, z }
            }
            Some("scale") => {
                assert_eq!(
                    transform.len(),
                    4,
                    "Expected scale to contain exactly 4 elements (including `scale`) at {:?}",
                    transform
                );
                let x = to_f64_lenient(&transform[1])?;
                let y = to_f64_lenient(&transform[2])?;
                let z = to_f64_lenient(&transform[3])?;
                Scale { x, y, z }
            }
            Some("rotate-x") => {
                assert_eq!(
                    transform.len(),
                    2,
                    "Expected rotate to contain a single value, in radians"
                );
                let rotation = to_f64_lenient(&transform[1])?;
                RotationX(rotation)
            }
            Some("rotate-y") => {
                assert_eq!(
                    transform.len(),
                    2,
                    "Expected rotate to contain a single value, in radians"
                );
                let rotation = to_f64_lenient(&transform[1])?;
                RotationY(rotation)
            }
            Some("rotate-z") => {
                assert_eq!(
                    transform.len(),
                    2,
                    "Expected rotate to contain a single value, in radians"
                );
                let rotation = to_f64_lenient(&transform[1])?;
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

        transforms.push(parsed)
    }

    Ok(transforms)
}

fn parse_object(yaml: &Yaml, kind: &str) -> Result<ObjectDescription, String> {
    let kind = match kind {
        "plane" => ObjectKind::Plane,
        "sphere" => ObjectKind::Sphere,
        "cube" => ObjectKind::Cube,
        _ => {
            return Err(format!(
                "cannot parse `{}` as Object (note: only primitives are supported)",
                kind
            ))
        }
    };

    let material = match &yaml["material"] {
        Yaml::String(reference) => Left(reference.to_owned()),
        description @ Yaml::Hash(_) => Right(parse_material(description)?),
        other => return Err(format!("cannot parse object material; expected an Object describing the material, or a String referencing a defined material, at {:?}", other))
    };

    let transform = parse_transform(&yaml["transform"])?;

    Ok(ObjectDescription {
        kind,
        material,
        transform,
    })
}

fn parse_tuple(yaml: &Yaml, key: &str) -> Result<(f64, f64, f64), String> {
    if let Some(components) = yaml[key].as_vec() {
        if components.len() != 3 {
            return Err(format!("Expected an array of exactly 3 numbers at {}", key));
        } else {
            let x = to_f64_lenient(&components[0])
                .map_err(|_| format!("Invalid `x` component of {}; expected number", key))?;
            let y = to_f64_lenient(&components[1])
                .map_err(|_| format!("Invalid `x` component of {}; expected number", key))?;
            let z = to_f64_lenient(&components[2])
                .map_err(|_| format!("Invalid `x` component of {}; expected number", key))?;

            Ok((x, y, z))
        }
    } else {
        Err(format!("Expected an array of exactly 3 numbers at {}", key))
    }
}

fn parse_usize(yaml: &Yaml, key: &str) -> Result<usize, String> {
    yaml[key]
        .as_i64()
        .ok_or(format!("Cannot parse required field {:?}", key))
        .and_then(|signed| {
            if signed < 0 {
                Err(format!("Field {:?} must not be negative", key))
            } else {
                Ok(signed as usize)
            }
        })
}

fn parse_f64(yaml: &Yaml, key: &str) -> Result<f64, String> {
    yaml[key]
        .as_f64()
        .ok_or(format!("Cannot parse required field {:?}", key))
}

fn to_f64_lenient(yaml: &Yaml) -> Result<f64, String> {
    match &yaml {
        // parsing can't actually fail here, the underlying YAML parser just converts lazily
        Yaml::Real(float) => Ok(float.parse::<f64>().unwrap()),
        Yaml::Integer(int) => Ok(*int as f64),
        _ => Err("Cannot parse as floating point".to_string()),
    }
}
