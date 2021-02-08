use crate::{Point3D, Vector3D};
use yaml_rust::{Yaml, YamlLoader};

#[cfg(test)]
mod tests;

pub fn parse(input: &str) -> Result<CameraDescription, String> {
    match YamlLoader::load_from_str(input) {
        Ok(yaml) => {
            let mut camera = None;

            for item in yaml[0] {
                if let Some("camera") = item["add"].as_str() {
                    camera = Some(parse_camera(&item)?);
                }
            }

            camera.ok_or("No `add: camera` directive found".to_string())
        }
        Err(error) => Err(error.to_string()),
    }
}

fn parse_camera(yaml: &Yaml) -> Result<CameraDescription, String> {
    let width = parse_usize(yaml, "width")?;
    let height = parse_usize(yaml, "height")?;
    let field_of_view = parse_f64(yaml, "field_of_view")?;

    Ok(CameraDescription {
        width,
        height,
        field_of_view,
        from: Point3D::ORIGIN,
        to: Point3D::ORIGIN,
        up: Vector3D::new(1.0, 0.0, 0.0),
    })
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

#[derive(PartialEq, Debug)]
pub struct CameraDescription {
    width: usize,
    height: usize,
    field_of_view: f64,
    from: Point3D,
    to: Point3D,
    up: Vector3D,
}
