use crate::{Object, Point3D, Vector, Vector3D};
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::str::SplitWhitespace;

#[cfg(test)]
mod tests;

pub fn parse(input: &str) -> ObjData {
    let mut vertices = vec![];
    let mut normals = vec![];
    let mut polys = vec![];
    let mut groups = vec![];
    let mut ignored_lines = 0;

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
            _ => ignored_lines += 1,
        }
    });

    if !polys.is_empty() {
        groups.push(polys)
    }

    ObjData {
        vertices,
        normals,
        groups,
        ignored_lines,
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

pub struct ObjData {
    vertices: Vec<Point3D>,
    normals: Vec<Vector3D>,
    groups: Vec<Group>,
    pub ignored_lines: usize,
}

impl ObjData {
    fn vertex(&self, index: usize) -> Option<Point3D> {
        self.vertices.get(index - 1).copied()
    }
    fn normal(&self, index: usize) -> Option<Vector3D> {
        self.normals.get(index - 1).copied()
    }
}

impl TryFrom<ObjData> for Object {
    type Error = String;

    fn try_from(obj_data: ObjData) -> Result<Self, Self::Error> {
        let convert_group = |group: &Group| {
            let mut triangles = vec![];

            for polygon in group {
                for face in triangulate(polygon) {
                    let mut vertices = Vec::with_capacity(3);
                    let mut normals = Vec::with_capacity(3);

                    for &(vert_index, normal_index) in face.iter() {
                        if let Some(vertex) = obj_data.vertex(vert_index) {
                            vertices.push(vertex)
                        } else {
                            return Err(format!(
                                "invalid vertex reference {} in face {:?}",
                                vert_index, polygon
                            ));
                        }

                        if let Some(normal_index) = normal_index {
                            if let Some(normal) = obj_data.normal(normal_index) {
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

        if obj_data.groups.len() == 1 {
            convert_group(&obj_data.groups[0])
        } else {
            let children = obj_data
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
