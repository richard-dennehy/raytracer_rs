use crate::{Object, Point3D};
use std::borrow::Borrow;
use std::convert::TryFrom;

#[cfg(test)]
mod tests;

pub fn parse(input: &str) -> ObjData {
    let mut vertices = vec![];
    let mut polys = vec![];
    let mut groups = vec![];
    let mut ignored_lines = 0;

    input
        .lines()
        .map(|line| line.trim())
        .for_each(|line| match line.chars().next() {
            Some('v') => vertices.push(parse_vertex(line)),
            Some('f') => polys.push(parse_polygon(line)),
            Some('g') => {
                if !polys.is_empty() {
                    let polys = std::mem::replace(&mut polys, vec![]);
                    groups.push(polys);
                }
            }
            _ => ignored_lines += 1,
        });

    if !polys.is_empty() {
        groups.push(polys)
    }

    ObjData {
        vertices,
        groups,
        ignored_lines,
    }
}

fn parse_vertex(line: &str) -> Point3D {
    let parts = line.split(' ').collect::<Vec<_>>();

    assert_eq!(
        parts.len(),
        4,
        "unparseable vertex data (expected 4 parts)\n{}",
        line
    );

    let get_float = |index: usize| {
        parts[index].parse::<f64>().expect(&format!(
            "unparseable vertex data (cannot parse component as f64)\n{}",
            line
        ))
    };

    Point3D::new(get_float(1), get_float(2), get_float(3))
}

fn parse_polygon(line: &str) -> Polygon {
    let parts = line.split(' ').collect::<Vec<_>>();

    assert!(
        parts.len() >= 4,
        "unparseable face data (expected at least 4 parts)\n{}",
        line
    );

    parts
        .into_iter()
        .skip(1)
        .map(|index| {
            index.parse::<usize>().expect(&format!(
                "unparseable vertex data (cannot parse component as f64)\n{}",
                line
            ))
        })
        .collect()
}

type Polygon = Vec<usize>;
type Group = Vec<Polygon>;

pub struct ObjData {
    vertices: Vec<Point3D>,
    groups: Vec<Group>,
    ignored_lines: usize,
}

impl ObjData {
    fn vertex(&self, index: usize) -> Option<Point3D> {
        self.vertices.get(index - 1).copied()
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

                    for &vert_index in face.iter() {
                        if let Some(vertex) = obj_data.vertex(vert_index) {
                            vertices.push(vertex)
                        } else {
                            return Err(format!(
                                "invalid vertex reference {} in face {:?}",
                                vert_index, polygon
                            ));
                        }
                    }

                    triangles.push(Object::triangle(vertices[0], vertices[1], vertices[2]))
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

fn triangulate(face: &Polygon) -> Vec<[usize; 3]> {
    let mut out = vec![];

    for i in 2..face.len() {
        out.push([face[0], face[i - 1], face[i]]);
    }

    out
}
