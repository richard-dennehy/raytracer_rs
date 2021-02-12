use crate::{Colour, Light, Point3D, Vector3D};
use yaml_rust::{Yaml, YamlLoader};

#[cfg(test)]
mod tests;

pub fn parse(input: &str) -> Result<SceneDescription, String> {
    match YamlLoader::load_from_str(input) {
        Ok(yaml) => {
            let mut camera = None;
            let mut lights = vec![];
            let mut defines = vec![];

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
                        Some(add) => todo!("add {}", add),
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

    let value = &yaml["value"];
    let value = if matches!(value, Yaml::Hash(_)) {
        parse_material(value)?
    } else {
        return Err(format!("cannot parse define at {}", name));
    };

    let extends = yaml["extend"].as_str().map(Into::into);

    Ok(Define {
        name,
        extends,
        value,
    })
}

fn parse_material(yaml: &Yaml) -> Result<Value, String> {
    let colour = parse_tuple(&yaml, "color").ok().map(Into::into);
    let diffuse = parse_f64(&yaml, "diffuse").ok();
    let ambient = parse_f64(&yaml, "ambient").ok();
    let specular = parse_f64(&yaml, "specular").ok();
    let shininess = parse_f64(&yaml, "shininess").ok();
    let reflective = parse_f64(&yaml, "reflective").ok();
    let transparency = parse_f64(&yaml, "transparency").ok();
    let refractive = parse_f64(&yaml, "refractive").ok();

    Ok(Value::Material {
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

#[derive(Debug, PartialEq)]
pub struct SceneDescription {
    pub camera: CameraDescription,
    pub lights: Vec<Light>,
    pub defines: Vec<Define>,
}

#[derive(PartialEq, Debug)]
pub struct CameraDescription {
    width: usize,
    height: usize,
    field_of_view: f64,
    from: Point3D,
    to: Point3D,
    up: Vector3D,
}

#[derive(PartialEq, Debug)]
pub struct Define {
    name: String,
    extends: Option<String>,
    value: Value,
}

#[derive(PartialEq, Debug)]
pub enum Value {
    Material {
        colour: Option<Colour>,
        diffuse: Option<f64>,
        ambient: Option<f64>,
        specular: Option<f64>,
        shininess: Option<f64>,
        reflective: Option<f64>,
        transparency: Option<f64>,
        refractive: Option<f64>,
    },
}
