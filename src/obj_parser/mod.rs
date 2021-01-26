use crate::{Object, Point3D};
use std::convert::TryFrom;

#[cfg(test)]
mod tests;

pub fn parse(input: &str) -> ObjData {
    let mut vertices = vec![];
    let mut faces = vec![];
    let mut ignored_lines = 0;

    input
        .lines()
        .map(|line| line.trim())
        .for_each(|line| match line.chars().next() {
            Some('v') => vertices.push(parse_vertex(line)),
            Some('f') => faces.push(parse_face(line)),
            _ => ignored_lines += 1,
        });

    ObjData {
        vertices,
        faces,
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

fn parse_face(line: &str) -> (usize, usize, usize) {
    let parts = line.split(' ').collect::<Vec<_>>();

    assert_eq!(
        parts.len(),
        4,
        "unparseable face data (expected 4 parts)\n{}",
        line
    );

    let get_index = |index: usize| {
        parts[index].parse::<usize>().expect(&format!(
            "unparseable vertex data (cannot parse component as f64)\n{}",
            line
        ))
    };

    (get_index(1), get_index(2), get_index(3))
}

pub struct ObjData {
    vertices: Vec<Point3D>,
    faces: Vec<(usize, usize, usize)>,
    ignored_lines: usize,
}

impl ObjData {
    fn vertex(&self, index: usize) -> Option<Point3D> {
        self.vertices.get(index - 1).copied()
    }
}

impl TryFrom<ObjData> for Object {
    type Error = String;

    fn try_from(value: ObjData) -> Result<Self, Self::Error> {
        let faces = value
            .faces
            .iter()
            .copied()
            .map(|(v1, v2, v3)| {
                let get_vertex = |index: usize| {
                    value
                        .vertex(index)
                        .ok_or_else(|| format!("face references invalid vertex {}", index))
                };

                get_vertex(v1).and_then(|v1| {
                    get_vertex(v2).and_then(|v2| get_vertex(v3).map(|v3| (v1, v2, v3)))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let tris = faces.into_iter().map(Object::triangle).collect();
        Ok(Object::group(tris))
    }
}
