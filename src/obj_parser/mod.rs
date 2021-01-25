use crate::Point3D;

#[cfg(test)]
mod tests;

pub fn parse(input: &str) -> ObjData {
    let mut vertices = vec![];
    let mut ignored_lines = 0;

    input
        .lines()
        .map(|line| line.trim())
        .for_each(|line| match line.chars().next() {
            Some('v') => vertices.push(parse_vertex(line)),
            _ => ignored_lines += 1,
        });

    ObjData {
        vertices,
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

pub struct ObjData {
    vertices: Vec<Point3D>,
    ignored_lines: usize,
}

impl ObjData {
    pub fn vertex(&self, index: usize) -> Option<Point3D> {
        self.vertices.get(index - 1).copied()
    }
}
