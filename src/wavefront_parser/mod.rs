use crate::{Colour, Material, MaterialKind, Object, Point3D, Vector, Vector3D};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::str::SplitWhitespace;

#[cfg(test)]
mod tests;

pub fn parse_mtl(input: &str) -> HashMap<String, Material> {
    MaterialParser {
        input,
        current: None,
        materials: HashMap::new(),
    }
    .parse()
}

struct MaterialParser<'input> {
    input: &'input str,
    current: Option<(&'input str, Material)>,
    materials: HashMap<String, Material>,
}

impl<'input> MaterialParser<'input> {
    fn parse(mut self) -> HashMap<String, Material> {
        self.input.lines().map(|line| line.trim()).for_each(|line| {
            let mut parts = line.split_whitespace();

            match parts.next() {
                Some("newmtl") => {
                    self.save_current_material();
                    self.current = Some((
                        parts
                            .next()
                            .expect("`newmtl` statement must provide a name"),
                        Material::default(),
                    ))
                }
                // note: the diffuse is meant to be an RGB colour value or a single greyscale value
                // but this doesn't match the ray tracer's internal representation of a material,
                // which has a single colour for all reflection components, and a _magnitude_ for the diffuse colour
                // and the most accurate conversion seems to be to parse the diffuse as the colour, and leave the default
                // diffuse strength of 0.9
                Some("Kd") => {
                    self.current_material().kind = MaterialKind::Solid(parse_colour(&mut parts))
                }
                // MTL ambience appears to be a percentage of the _scene_ ambience, which doesn't match the
                // way the ray tracer models ambience - parsing MTL values directly to material `ambient` will
                // result in incredibly bright materials that don't interact with light as intended, so
                // multiplying by 0.1 adjusts the range
                Some("Ka") => self.current_material().ambient = parse_rgb_to_f64(&mut parts) * 0.1,
                Some("Ks") => self.current_material().specular = parse_rgb_to_f64(&mut parts),
                Some("Ns") => {
                    if let Some(shininess) = parts.next().and_then(|s| s.parse::<f64>().ok()) {
                        self.current_material().shininess = shininess
                    }
                }
                Some("Ni") => {
                    if let Some(refractive) = parts.next().and_then(|r| r.parse::<f64>().ok()) {
                        self.current_material().refractive = refractive
                    }
                }
                Some("d") => {
                    if let Some(dissolve) = parts.next().and_then(|d| d.parse::<f64>().ok()) {
                        self.current_material().transparency = 1.0 - dissolve
                    }
                }
                Some("illum") => match parts.next() {
                    Some("0") => {
                        self.current_material().ambient = 1.0;
                        self.current_material().diffuse = 0.0;
                        self.current_material().specular = 0.0;
                    }
                    Some("1") => {
                        self.current_material().specular = 0.0;
                    }
                    Some("2") => (),
                    Some("3" | "8") => {
                        if self.current_material().reflective == 0.0 {
                            self.current_material().reflective = 1.0
                        }
                    }
                    Some("4" | "5" | "6" | "7") => {
                        if self.current_material().reflective == 0.0 {
                            self.current_material().reflective = 1.0;
                        }
                        if self.current_material().transparency == 0.0 {
                            self.current_material().transparency = 1.0;
                        }
                    }
                    Some("9") => {
                        if self.current_material().transparency == 0.0 {
                            self.current_material().transparency = 1.0
                        }
                    }
                    Some("10") => panic!("illum model 10 is not supported"),
                    Some(other) => {
                        panic!("invalid illum value {} - must be between 0 and 10", other)
                    }
                    None => panic!("illum does not have a value"),
                },
                _ => (),
            }
        });

        self.save_current_material();
        self.materials
    }

    fn save_current_material(&mut self) {
        if let Some((name, material)) = self.current.take() {
            if let Some(_) = self.materials.insert(name.to_owned(), material) {
                panic!("duplicate material with name {}", name)
            }
        };
    }

    fn current_material(&mut self) -> &mut Material {
        if let Some((_, material)) = &mut self.current {
            material
        } else {
            panic!("A material must be defined with a `newmtl` statement before material properties can be defined")
        }
    }
}

fn parse_colour(iterator: &mut SplitWhitespace) -> Colour {
    match iterator.next() {
        Some("spectral" | "xyz") => panic!("only RGB statements are supported"),
        None => panic!("statement does not specify an RGB colour"),
        Some(r) => {
            let red = r
                .parse::<f64>()
                .expect("unparseable colour component (expected valid f64)");
            let green = iterator.next().and_then(|g| g.parse::<f64>().ok());
            let blue = iterator.next().and_then(|b| b.parse::<f64>().ok());

            if let Some(green) = green {
                if let Some(blue) = blue {
                    Colour::new(red, green, blue)
                } else {
                    panic!("Invalid RGB colour in statement - must either specify 1 f64 value or 3")
                }
            } else {
                Colour::greyscale(red)
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
fn parse_rgb_to_f64(iterator: &mut SplitWhitespace) -> f64 {
    let colour = parse_colour(iterator);

    if colour.green() == 0.0 && colour.blue() == 0.0 {
        colour.red()
    } else if colour.red() == 0.0 && colour.blue() == 0.0 {
        colour.green()
    } else if colour.red() == 0.0 && colour.green() == 0.0 {
        colour.blue()
    } else {
        (colour.red() + colour.green() + colour.blue()) / 3.0
    }
}

pub fn parse_obj(input: &str) -> ObjData {
    let mut vertices = vec![];
    let mut normals = vec![];
    let mut polys = vec![];
    let mut groups = vec![];

    input.lines().map(|line| line.trim()).for_each(|line| {
        let mut parts = line.split_whitespace();

        match parts.next() {
            Some("v") => vertices.push(parse_vertex(parts)),
            Some("f") => polys.push(parse_polygon(parts)),
            Some("vn") => normals.push(parse_normal(parts)),
            Some("g") => {
                if !polys.is_empty() {
                    let polys = std::mem::replace(&mut polys, vec![]);
                    groups.push(polys);
                }
            }
            _ => (),
        }
    });

    if !polys.is_empty() {
        groups.push(polys)
    }

    ObjData {
        vertices,
        normals,
        groups,
    }
}

fn parse_vertex(mut line_parts: SplitWhitespace) -> Point3D {
    let mut next = || {
        let part = line_parts.next().expect("missing line part");
        part.parse::<f64>().expect(&format!(
            "unparseable vertex data (cannot parse component as f64)\n{}",
            part
        ))
    };

    Point3D::new(next(), next(), next())
}

fn parse_polygon(line_parts: SplitWhitespace) -> Polygon {
    fn parse_usize(s: &str) -> usize {
        s.parse::<usize>().expect(&format!(
            "Unparseable polygon data (cannot parse component as integer): {}",
            s
        ))
    }

    line_parts
        .map(|part| {
            let mut parts = part.split('/');
            let vertex = parts
                .next()
                .expect(&format!("Invalid polygon data: {}", part));
            let vertex = parse_usize(vertex);

            let mut next = || parts.next().filter(|&s| !s.is_empty()).map(parse_usize);

            let texture_vertex = next();
            let normal = next();

            PolygonData {
                vertex,
                texture_vertex,
                normal,
            }
        })
        .collect()
}

fn parse_normal(mut line_parts: SplitWhitespace) -> Vector3D {
    let mut next = || {
        let part = line_parts.next().expect("missing line part");
        part.parse::<f64>().expect(&format!(
            "unparseable normal data (cannot parse component as f64)\n{}",
            part
        ))
    };

    Vector3D::new(next(), next(), next())
}

type Polygon = Vec<PolygonData>;
type Group = Vec<Polygon>;

#[allow(unused)]
#[derive(Debug, Eq, PartialEq)]
struct PolygonData {
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

    pub fn to_object(&self) -> Result<Object, String> {
        let convert_group = |group: &Group| {
            let mut triangles = vec![];

            for polygon in group {
                for face in triangulate(polygon) {
                    let mut vertices = Vec::with_capacity(3);
                    let mut normals = Vec::with_capacity(3);

                    for &(vert_index, normal_index) in face.iter() {
                        if let Some(vertex) = self.vertex(vert_index) {
                            vertices.push(vertex)
                        } else {
                            return Err(format!(
                                "invalid vertex reference {} in face {:?}",
                                vert_index, polygon
                            ));
                        }

                        if let Some(normal_index) = normal_index {
                            if let Some(normal) = self.normal(normal_index) {
                                normals.push(normal)
                            } else {
                                return Err(format!(
                                    "invalid normal reference {} in face {:?}",
                                    normal_index, polygon
                                ));
                            }
                        }
                    }

                    if normals.is_empty() {
                        triangles.push(Object::triangle(vertices[0], vertices[1], vertices[2]))
                    } else if normals.len() == 3 {
                        triangles.push(Object::smooth_triangle(
                            vertices[0],
                            vertices[1],
                            vertices[2],
                            // should probably refuse to parse a file with bad normals, but floating point errors may make a normal in the file non-normalised after parsing
                            normals[0].normalised(),
                            normals[1].normalised(),
                            normals[2].normalised(),
                        ))
                    } else {
                        return Err(format!(
                            "Face {:?} must either have normals for all faces or no faces",
                            polygon
                        ));
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

fn triangulate(face: &Polygon) -> Vec<[(usize, Option<usize>); 3]> {
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
