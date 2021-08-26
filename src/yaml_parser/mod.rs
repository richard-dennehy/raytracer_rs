use yaml_rust::{Yaml, YamlLoader};

use anyhow::*;
use model::*;
use parsers::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod tests;

mod model;
mod parsers;

pub fn load(resource_dir: PathBuf, file_name: &str) -> anyhow::Result<SceneDescription> {
    let yaml = fs::read_to_string(resource_dir.join(file_name))?;
    parse(&yaml, resource_dir)
}

fn parse(input: &str, resource_dir: PathBuf) -> anyhow::Result<SceneDescription> {
    match YamlLoader::load_from_str(input) {
        Ok(yaml) => {
            let mut camera = None;
            let mut lights = vec![];
            let mut new_defines = HashMap::new();
            let mut objects = vec![];

            if let Some(items) = yaml[0].as_vec() {
                for item in items {
                    let item = ParseState::new(item, &new_defines);
                    match item.get("add").as_str() {
                        Some("camera") => {
                            camera = Some(
                                item.with_context("add")
                                    .with_extra_context("camera".into())
                                    .parse()?,
                            );
                            continue;
                        }
                        Some("light") => {
                            lights.push(
                                item.with_context("add")
                                    .with_extra_context("light".into())
                                    .parse()?,
                            );
                            continue;
                        }
                        Some(add) => {
                            objects.push(
                                item.with_context("add")
                                    .with_extra_context(add.into())
                                    .parse()?,
                            );
                            continue;
                        }
                        None => (),
                    }

                    if let Some(name) = item.get("define").as_str() {
                        let define = item
                            .with_context("define")
                            .with_extra_context(name.into())
                            .parse()?;
                        let name = name.to_owned();

                        if let Some(_) = new_defines.insert(name.clone(), define) {
                            bail!("duplicate `define` with name {:?}", name);
                        }

                        continue;
                    }
                }
            } else {
                bail!("Expected a list of directives");
            }

            let camera = camera.ok_or(anyhow!("No `add: camera` directive found"))?;

            Ok(SceneDescription {
                camera,
                lights,
                objects,
                resource_dir,
            })
        }
        Err(error) => bail!(error),
    }
}

struct ParseState<'yaml> {
    current: &'yaml Yaml,
    context: &'yaml str,
    extra_context: Option<String>,
    pub defines: &'yaml Defines,
}

impl<'yaml> ParseState<'yaml> {
    pub fn new(current: &'yaml Yaml, defines: &'yaml Defines) -> Self {
        ParseState {
            current,
            defines,
            context: "",
            extra_context: None,
        }
    }

    pub fn parse<T: FromYaml>(&self) -> anyhow::Result<T> {
        let extra_context = match &self.extra_context {
            Some(ctx) => format!(" [{}]", ctx),
            None => "".into(),
        };

        T::from_yaml(&self).with_context(|| {
            format!(
                "cannot parse `{}`{} as {}",
                self.context,
                extra_context,
                T::type_name()
            )
        })
    }

    pub fn get(&self, field: &'static str) -> Self {
        ParseState {
            current: &self.current[field],
            context: field,
            extra_context: None,
            defines: self.defines,
        }
    }

    pub fn as_str(&self) -> Option<&'yaml str> {
        self.current.as_str()
    }

    pub fn as_vec(&self) -> Option<Vec<ParseState>> {
        self.current.as_vec().map(|v| {
            v.iter()
                .enumerate()
                .map(|(idx, item)| ParseState {
                    current: item,
                    context: self.context,
                    extra_context: Some(format!("index {}", idx)),
                    defines: self.defines,
                })
                .collect()
        })
    }

    pub fn yaml(&self) -> &Yaml {
        self.current
    }

    pub fn with_context(self, context: &'yaml str) -> Self {
        Self { context, ..self }
    }

    pub fn with_extra_context(self, context: String) -> Self {
        Self {
            extra_context: Some(context),
            ..self
        }
    }
}
