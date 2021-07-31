use super::{Pattern, UvPattern};
use crate::core::Colour;

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub kind: MaterialKind,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive: f64,
    pub casts_shadow: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MaterialKind {
    Pattern(Pattern),
    Uv(UvPattern),
    Solid(Colour),
}

impl Default for Material {
    fn default() -> Self {
        Material {
            kind: MaterialKind::Solid(Colour::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
            casts_shadow: true,
        }
    }
}
