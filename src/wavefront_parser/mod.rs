use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::SplitWhitespace;

use crate::core::{Colour, Point3D, Vector3D, VectorMaths};
use crate::scene::Object;
use crate::scene::{Material, MaterialKind};
use anyhow::*;
use std::cell::RefCell;

#[cfg(test)]
mod tests;

type Cache<T> = RefCell<HashMap<String, T>>;

pub struct WavefrontParser {
    mtl_cache: Cache<Materials>,
    obj_cache: Cache<ObjData>,
    resource_path: PathBuf,
}

impl WavefrontParser {
    pub fn new(resource_path: PathBuf) -> Self {
        Self {
            mtl_cache: RefCell::new(HashMap::new()),
            obj_cache: RefCell::new(HashMap::new()),
            resource_path,
        }
    }

    pub fn load(&self, file_name: &str) -> anyhow::Result<Object> {
        if !file_name.ends_with(".obj") {
            bail!("{} is not a .obj file", file_name);
        };

        if let Some(obj_data) = self.obj_cache.borrow().get(file_name) {
            return obj_data.to_object();
        }

        let file = self.resource_path.join(&file_name);
        println!("loading OBJ file {}", file.to_str().unwrap());
        let contents = fs::read_to_string(file)?;

        self.load_mtl_libraries(&contents)?;
        let obj_data = self.parse_obj(&contents);

        self.obj_cache
            .borrow_mut()
            .entry(file_name.to_string())
            .or_insert(obj_data?)
            .to_object()
    }

    fn load_mtl_libraries(&self, file: &str) -> anyhow::Result<()> {
        file.lines()
            .map(|line| line.trim())
            .map(|line| match line.split_whitespace().next() {
                Some("mtllib") => {
                    let file_names = line.chars().skip("mttlib ".len()).collect::<String>();
                    let file_names = file_names
                        .split(".mtl")
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>();

                    file_names
                        .into_iter()
                        .map(|file_name| self.load_mtl(file_name))
                        .collect::<anyhow::Result<()>>()
                }
                _ => Ok(()),
            })
            .collect::<anyhow::Result<()>>()?;

        Ok(())
    }

    fn load_mtl(&self, file_name: &str) -> anyhow::Result<()> {
        if let Some(_) = self.mtl_cache.borrow().get(file_name) {
            return Ok(());
        }

        let file = self.resource_path.join(format!("{}.mtl", file_name));
        println!("loading MTL file {}", file.to_str().unwrap());
        let contents = fs::read_to_string(file)?;

        self.mtl_cache
            .borrow_mut()
            .insert(file_name.to_string(), parse_mtl(&contents)?);

        Ok(())
    }

    fn parse_obj(&self, input: &str) -> anyhow::Result<ObjData> {
        let material_cache = self.mtl_cache.borrow();

        let mut vertices = vec![];
        let mut normals = vec![];
        let mut polys = vec![];
        let mut groups = vec![];
        let mut loaded_materials = HashMap::new();
        let mut current_material: Option<&Material> = None;

        input
            .lines()
            .map(|line| line.trim())
            .map(|line| {
                let mut parts = line.split_whitespace();

                match parts.next() {
                    Some("mtllib") => {
                        let file_names = line.chars().skip("mttlib ".len()).collect::<String>();
                        let materials = file_names
                            .split(".mtl")
                            .map(str::trim)
                            .filter(|s| !s.is_empty())
                            .map(|file_name| {
                                material_cache.get(file_name).ok_or_else(|| {
                                    anyhow!(
                                        "material library `{}` must be loaded before obj file can be parsed",
                                        file_name
                                    )
                                })
                            })
                            .collect::<anyhow::Result<Vec<_>>>();

                        materials.map(|materials| {
                            materials.into_iter().flat_map(|m| m.0.iter()).for_each(|(material_name, material)| {
                                loaded_materials
                                    .entry(material_name.as_str())
                                    .or_insert(material);
                            })
                        })
                    }
                    Some("v") => parse_vertex(parts).map(|v| vertices.push(v)),
                    Some("f") => parse_polygon(parts, current_material.cloned()).map(|p| polys.push(p)),
                    Some("vn") => parse_normal(parts).map(|n| normals.push(n)),
                    Some("g") => {
                        if !polys.is_empty() {
                            let polygons = std::mem::take(&mut polys);
                            groups.push(Group { polygons });
                        }

                        Ok(())
                    }
                    Some("usemtl") => {
                        parts.next().ok_or_else(|| anyhow!("`usemtl` statement does not name the material")).and_then(|name| {
                            let loaded = loaded_materials.get(name).ok_or_else(|| anyhow!(
                                "cannot `usemtl {}` as it has not been loaded from an MTL library",
                                name
                            ));

                            loaded.map(|l| current_material = Some(*l))
                        })
                    }
                    _ => Ok(()),
                }
            })
            .collect::<anyhow::Result<()>>()?;

        if !polys.is_empty() {
            groups.push(Group { polygons: polys })
        }

        Ok(ObjData {
            vertices,
            normals,
            groups,
        })
    }
}

pub fn parse_mtl(input: &str) -> anyhow::Result<Materials> {
    MaterialParser {
        input,
        current: None,
        materials: HashMap::new(),
    }
    .parse()
}

#[derive(Debug, PartialEq)]
pub struct Materials(HashMap<String, Material>);

impl Materials {
    pub fn get(&self, key: &str) -> Option<&Material> {
        self.0.get(key)
    }
}

struct MaterialParser<'input> {
    input: &'input str,
    current: Option<(&'input str, Material)>,
    materials: HashMap<String, Material>,
}

impl<'input> MaterialParser<'input> {
    fn parse(mut self) -> anyhow::Result<Materials> {
        self.input
            .lines()
            .map(|line| line.trim())
            .map(|line| {
                let mut parts = line.split_whitespace();

                match parts.next() {
                    Some("newmtl") => {
                        self.save_current_material()?;
                        self.current = Some((
                            parts
                                .next()
                                .ok_or_else(|| anyhow!("`newmtl` statement must provide a name"))?,
                            Material::default(),
                        ))
                    }
                    // note: the diffuse is meant to be an RGB colour value or a single greyscale value
                    // but this doesn't match the ray tracer's internal representation of a material,
                    // which has a single colour for all reflection components, and a _magnitude_ for the diffuse colour
                    // and the most accurate conversion seems to be to parse the diffuse as the colour, and leave the default
                    // diffuse strength of 0.9
                    Some("Kd") => {
                        self.current_material()?.kind =
                            MaterialKind::Solid(parse_colour(&mut parts)?)
                    }
                    // MTL ambience appears to be a percentage of the _scene_ ambience, which doesn't match the
                    // way the ray tracer models ambience - parsing MTL values directly to material `ambient` will
                    // result in incredibly bright materials that don't interact with light as intended, so
                    // multiplying by 0.1 adjusts the range
                    Some("Ka") => {
                        self.current_material()?.ambient = parse_rgb_to_f64(&mut parts)? * 0.1
                    }
                    Some("Ks") => self.current_material()?.specular = parse_rgb_to_f64(&mut parts)?,
                    Some("Ns") => {
                        if let Some(shininess) = parts.next().and_then(|s| s.parse::<f64>().ok()) {
                            self.current_material()?.shininess = shininess
                        }
                    }
                    Some("Ni") => {
                        if let Some(refractive) = parts.next().and_then(|r| r.parse::<f64>().ok()) {
                            self.current_material()?.refractive = refractive
                        }
                    }
                    Some("d") => {
                        if let Some(dissolve) = parts.next().and_then(|d| d.parse::<f64>().ok()) {
                            self.current_material()?.transparency = 1.0 - dissolve
                        }
                    }
                    Some("illum") => self.apply_illumination(parts.next())?,
                    _ => (),
                };

                Ok(())
            })
            .collect::<anyhow::Result<()>>()?;

        self.save_current_material()?;
        Ok(Materials(self.materials))
    }

    fn save_current_material(&mut self) -> anyhow::Result<()> {
        if let Some((name, material)) = self.current.take() {
            if let Some(_) = self.materials.insert(name.to_owned(), material) {
                bail!("duplicate material with name `{}`", name);
            }
        };

        return Ok(());
    }

    fn current_material(&mut self) -> anyhow::Result<&mut Material> {
        if let Some((_, material)) = &mut self.current {
            Ok(material)
        } else {
            bail!("A material must be defined with a `newmtl` statement before material properties can be defined");
        }
    }

    fn apply_illumination(&mut self, illum: Option<&str>) -> anyhow::Result<()> {
        match illum {
            Some("0") => {
                self.current_material()?.ambient = 1.0;
                self.current_material()?.diffuse = 0.0;
                self.current_material()?.specular = 0.0;
            }
            Some("1") => {
                self.current_material()?.specular = 0.0;
            }
            Some("2") => (),
            Some("3" | "8") => {
                if self.current_material()?.reflective == 0.0 {
                    self.current_material()?.reflective = 1.0
                }
            }
            Some("4" | "5" | "6" | "7") => {
                if self.current_material()?.reflective == 0.0 {
                    self.current_material()?.reflective = 1.0;
                }
                if self.current_material()?.transparency == 0.0 {
                    self.current_material()?.transparency = 1.0;
                }
            }
            Some("9") => {
                if self.current_material()?.transparency == 0.0 {
                    self.current_material()?.transparency = 1.0
                }
            }
            Some("10") => bail!("illum model 10 is not supported"),
            Some(other) => {
                bail!("invalid illum value `{}` - must be between 0 and 10", other)
            }
            None => bail!("illum does not have a value"),
        };

        Ok(())
    }
}

fn parse_colour(iterator: &mut SplitWhitespace) -> anyhow::Result<Colour> {
    match iterator.next() {
        Some("spectral" | "xyz") => bail!("only RGB statements are supported"),
        None => bail!("statement does not specify an RGB colour"),
        Some(r) => {
            let red = r
                .parse::<f64>()
                .map_err(|e| anyhow!("unparseable colour component ({})", e.to_string()))?;
            let green = iterator.next().and_then(|g| g.parse::<f64>().ok());
            let blue = iterator.next().and_then(|b| b.parse::<f64>().ok());

            if let Some(green) = green {
                if let Some(blue) = blue {
                    Ok(Colour::new(red, green, blue))
                } else {
                    bail!("Invalid RGB colour in statement - must either specify 1 f64 value or 3")
                }
            } else {
                Ok(Colour::greyscale(red))
            }
        }
    }
}

/// given that the various `K` statements (`Ka`, `Ks`, etc) each permit either 3 separate values
/// representing a colour, or one value representing a strength, but the ray tracer only allows
/// the strength of each component to be specified, this tries to convert a colour value (including
/// a greyscale value generated by a single f64) into a strength on a best-effort basis
///
/// - if only one value is specified e.g. `Ka 1`, that value will be returned
/// - if three equal values are specified, e.g. `Ka 1 1 1`, the first value will be returned
/// - if two zero values and a nonzero value are specified e.g. `Ka 0 1 0`, the nonzero value will be returned
/// - if three different values are specified, then the ray tracer cannot accurately represent this, but the values will be averaged to attempt a vaguely accurate portrayal
fn parse_rgb_to_f64(iterator: &mut SplitWhitespace) -> anyhow::Result<f64> {
    let colour = parse_colour(iterator)?;

    if colour.green() == 0.0 && colour.blue() == 0.0 {
        Ok(colour.red())
    } else if colour.red() == 0.0 && colour.blue() == 0.0 {
        Ok(colour.green())
    } else if colour.red() == 0.0 && colour.green() == 0.0 {
        Ok(colour.blue())
    } else {
        Ok((colour.red() + colour.green() + colour.blue()) / 3.0)
    }
}

fn parse_vertex(mut line_parts: SplitWhitespace) -> anyhow::Result<Point3D> {
    let mut next = || {
        line_parts
            .next()
            .ok_or_else(|| anyhow!("missing line part"))
            .and_then(|part| {
                part.parse::<f64>()
                    .map_err(|e| anyhow!("unparseable vertex data `{}` ({})", part, e.to_string(),))
            })
    };

    Ok(Point3D::new(next()?, next()?, next()?))
}

fn parse_polygon(
    line_parts: SplitWhitespace,
    material: Option<Material>,
) -> anyhow::Result<Polygon> {
    fn parse_usize(s: &str) -> anyhow::Result<usize> {
        s.parse::<usize>()
            .map_err(|e| anyhow!("Unparseable polygon data `{}` ({})", s, e.to_string()))
    }

    let vertices = line_parts
        .map(|part| {
            let mut parts = part.split('/');
            let vertex = parts
                .next()
                .ok_or_else(|| anyhow!("Invalid polygon data: `{}`", part))?;
            let vertex = parse_usize(vertex)?;

            let mut next = || {
                parts
                    .next()
                    .filter(|&s| !s.is_empty())
                    .map(parse_usize)
                    .transpose()
            };

            let texture_vertex = next()?;
            let normal = next()?;

            Ok(VertexData {
                vertex,
                texture_vertex,
                normal,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(Polygon { vertices, material })
}

fn parse_normal(mut line_parts: SplitWhitespace) -> anyhow::Result<Vector3D> {
    let mut next = || {
        line_parts
            .next()
            .ok_or_else(|| anyhow!("missing line part"))
            .and_then(|part| {
                part.parse::<f64>()
                    .map_err(|e| anyhow!("unparseable normal data `{}` ({})", part, e.to_string()))
            })
    };

    Ok(Vector3D::new(next()?, next()?, next()?))
}

#[derive(Debug, PartialEq)]
struct Polygon {
    vertices: Vec<VertexData>,
    material: Option<Material>,
}

#[derive(Debug, PartialEq)]
struct Group {
    polygons: Vec<Polygon>,
}

#[derive(Debug, PartialEq)]
struct VertexData {
    vertex: usize,
    texture_vertex: Option<usize>,
    normal: Option<usize>,
}

#[derive(Debug, PartialEq)]
pub struct ObjData {
    vertices: Vec<Point3D>,
    normals: Vec<Vector3D>,
    groups: Vec<Group>,
}

impl ObjData {
    fn vertex(&self, index: usize) -> Option<Point3D> {
        self.vertices.get(index - 1).copied()
    }
    fn normal(&self, index: usize) -> Option<Vector3D> {
        self.normals.get(index - 1).copied()
    }

    pub fn to_object(&self) -> anyhow::Result<Object> {
        let convert_group = |group: &Group| {
            let mut triangles = vec![];

            for polygon in &group.polygons {
                for face in triangulate(&polygon.vertices) {
                    let mut vertices = Vec::with_capacity(3);
                    let mut normals = Vec::with_capacity(3);

                    for &(vert_index, normal_index) in face.iter() {
                        if let Some(vertex) = self.vertex(vert_index) {
                            vertices.push(vertex)
                        } else {
                            bail!(
                                "invalid vertex reference `{}` in face {:?}",
                                vert_index,
                                polygon
                            );
                        }

                        if let Some(normal_index) = normal_index {
                            if let Some(normal) = self.normal(normal_index) {
                                normals.push(normal)
                            } else {
                                bail!(
                                    "invalid normal reference `{}` in face {:?}",
                                    normal_index,
                                    polygon
                                );
                            }
                        }
                    }

                    let triangle = if normals.is_empty() {
                        Object::triangle(vertices[0], vertices[1], vertices[2])
                    } else if normals.len() == 3 {
                        Object::smooth_triangle(
                            vertices[0],
                            vertices[1],
                            vertices[2],
                            // should probably refuse to parse a file with bad normals, but floating point errors may make a normal in the file non-normalised after parsing
                            normals[0].normalised(),
                            normals[1].normalised(),
                            normals[2].normalised(),
                        )
                    } else {
                        bail!(
                            "Face {:?} must either have normals for all faces or no faces",
                            polygon
                        );
                    };

                    if let Some(material) = &polygon.material {
                        triangles.push(triangle.with_material(material.clone()))
                    } else {
                        triangles.push(triangle)
                    }
                }
            }

            Ok(Object::group(triangles))
        };

        if self.groups.len() == 1 {
            convert_group(&self.groups[0])
        } else {
            let children = self
                .groups
                .iter()
                .map(Borrow::borrow)
                .map(convert_group)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Object::group(children))
        }
    }
}

fn triangulate(face: &[VertexData]) -> Vec<[(usize, Option<usize>); 3]> {
    let mut out = vec![];

    for i in 2..face.len() {
        out.push([
            (face[0].vertex, face[0].normal),
            (face[i - 1].vertex, face[i - 1].normal),
            (face[i].vertex, face[i].normal),
        ]);
    }

    out
}
