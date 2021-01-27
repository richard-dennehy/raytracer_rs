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

fn parse_face(line: &str) -> Vec<usize> {
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

pub struct ObjData {
    vertices: Vec<Point3D>,
    faces: Vec<Vec<usize>>,
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
        fn triangulate(faces: &Vec<usize>) -> Vec<[usize; 3]> {
            let mut out = vec![];

            for i in 2..faces.len() {
                out.push([faces[0], faces[i - 1], faces[i]]);
            }

            out
        }

        let faces = obj_data
            .faces
            .iter()
            .flat_map(|verts| {
                let get_vertex = |index: usize| {
                    obj_data
                        .vertex(index)
                        .ok_or_else(|| format!("face references invalid vertex {}", index))
                };

                triangulate(verts)
                    .iter()
                    .map(|tris| {
                        tris.iter()
                            .copied()
                            .map(get_vertex)
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .map(|result| result.map(|tris| (tris[0], tris[1], tris[2])))
                    .collect::<Vec<_>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let tris = faces
            .into_iter()
            .map(|(v1, v2, v3)| Object::triangle(v1, v2, v3))
            .collect();
        Ok(Object::group(tris))
    }
}
