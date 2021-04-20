use yaml_rust::YamlLoader;

use model::*;
use parsers::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

mod model;
mod parsers;

pub fn parse(input: &str) -> Result<SceneDescription, String> {
    match YamlLoader::load_from_str(input) {
        Ok(yaml) => {
            let mut camera = None;
            let mut lights = vec![];
            let mut new_defines = HashMap::new();
            let mut objects = vec![];

            if let Some(items) = yaml[0].as_vec() {
                for item in items {
                    match item["add"].as_str() {
                        Some("camera") => {
                            camera = Some(item.parse(&new_defines)?);
                            continue;
                        }
                        Some("light") => {
                            lights.push(item.parse(&new_defines)?);
                            continue;
                        }
                        Some(_) => {
                            objects.push(item.parse(&new_defines)?);
                            continue;
                        }
                        None => (),
                    }

                    if let Some(name) = item["define"].as_str() {
                        if let Some(_) =
                            new_defines.insert(name.to_owned(), item.parse(&new_defines)?)
                        {
                            return Err(format!("duplicate `define` with name {:?}", name));
                        }

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
                objects,
            })
        }
        Err(error) => Err(error.to_string()),
    }
}
