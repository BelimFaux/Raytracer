use crate::{math::Point3, objects::Triangle};

use super::InputError;

/// three positive integers
type Triple = (u32, u32, u32);

/// parses an `.obj` file to a list of triangles
pub fn parse(src: String) -> Result<Vec<Triangle>, InputError> {
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut triangles = Vec::new();

    for (current_line, line) in src.lines().enumerate() {
        let mut words = line.trim().split(" ");
        if let Some(t) = words.next() {
            match t {
                "v" => {
                    vertices.push(parse_point(words.collect()).map_err(|s| err(current_line, &s))?)
                }
                "vn" => {
                    normals.push(parse_point(words.collect()).map_err(|s| err(current_line, &s))?)
                }
                "f" => {
                    let (verts, norm) =
                        parse_face(words.collect()).map_err(|s| err(current_line, &s))?;

                    let tri = Triangle::new(
                        get_elements(&vertices, verts).map_err(|s| err(current_line, &s))?,
                        get_elements(&normals, norm).map_err(|s| err(current_line, &s))?,
                    );
                    triangles.push(tri);
                }
                _ => {}
            }
        }
    }

    Ok(triangles)
}

/// Get 3 Points from a slice of points using a triple of indices
fn get_elements(from: &[Point3], indices: Triple) -> Result<[Point3; 3], String> {
    Ok([
        *from
            .get((indices.0 - 1) as usize)
            .ok_or(format!("Invalid index {} for face data", indices.0))?,
        *from
            .get((indices.1 - 1) as usize)
            .ok_or(format!("Invalid index {} for face data", indices.1))?,
        *from
            .get((indices.2 - 1) as usize)
            .ok_or(format!("Invalid index {} for face data", indices.2))?,
    ])
}

/// parse a face line in the format:
/// `v/vt/vn v/vt/vn v/vt/vn`
/// where `v` is the vertex index, `vt` is the texture index and `vn` is the normal index
fn parse_face(line: Vec<&str>) -> Result<(Triple, Triple), String> {
    if line.len() != 3 {
        return Err(format!("Expected 3 elements but got {}", line.len()));
    }

    let mut vertices = [0, 0, 0];
    let mut normals = [0, 0, 0];

    for (i, elem) in line.iter().enumerate() {
        let mut parts = elem.split("/");
        let (v, _t, n) = (parts.next(), parts.next(), parts.next());
        if parts.next().is_some() {
            return Err(String::from("Face data contains more than 3 elements"));
        }
        vertices[i] = v
            .ok_or(String::from("Expected vertices data"))?
            .parse::<u32>()
            .map_err(|r| r.to_string())?;

        normals[i] = n
            .ok_or(String::from("Expected vertices data"))?
            .parse::<u32>()
            .map_err(|r| r.to_string())?;
    }

    Ok((vertices.into(), normals.into()))
}

/// parse a single point in the format: `x y z`
fn parse_point(line: Vec<&str>) -> Result<Point3, String> {
    if line.len() != 3 {
        return Err(format!("Expected 3 elements but got {}", line.len()));
    }

    let (x, y, z) = (&line[0], &line[1], &line[2]);

    Ok(Point3::new(
        x.parse::<f32>().map_err(|r| r.to_string())?,
        y.parse::<f32>().map_err(|r| r.to_string())?,
        z.parse::<f32>().map_err(|r| r.to_string())?,
    ))
}

/// construct an appropriate error message
fn err(current_line: usize, msg: &str) -> InputError {
    InputError(format!("Error on line {}: {msg}", current_line + 1))
}

#[cfg(test)]
mod tests {
    use crate::math::Vector3;

    use super::*;

    fn vec_cmp(lhs: &[Triangle], rhs: &[Triangle]) -> bool {
        (lhs.len() == rhs.len()) && lhs.iter().zip(rhs).all(|(l, r)| l == r)
    }

    #[test]
    fn parse_objectfile_expect_plane_triangles() {
        let filecontents = r#"
            # Blender3D v249 OBJ File: 
            # www.blender3d.org
            v 1.000000 1.000000 0.000000
            v 1.000000 -1.000000 0.000000
            v -1.000000 -1.000000 0.000000
            v -1.000000 1.000000 0.000000
            vt 0.000000 0.000000
            vt 10.000000 0.000000
            vt 10.000000 10.000000
            vt 0.000000 10.000000
            vn 0.000000 0.000000 1.000000
            usemtl (null)
            s off
            f 1/1/1 4/2/1 3/3/1
            f 1/1/1 3/3/1 2/4/1
        "#
        .to_string();

        let mesh = parse(filecontents);

        assert!(mesh.is_ok());

        let triangles = mesh.unwrap();

        let expected = vec![
            Triangle::new(
                [
                    Point3::new(1., 1., 0.),
                    Point3::new(-1., 1., 0.),
                    Point3::new(-1., -1., 0.),
                ],
                [
                    Vector3::new(0., 0., 1.),
                    Vector3::new(0., 0., 1.),
                    Vector3::new(0., 0., 1.),
                ],
            ),
            Triangle::new(
                [
                    Point3::new(1., 1., 0.),
                    Point3::new(-1., -1., 0.),
                    Point3::new(1., -1., 0.),
                ],
                [
                    Vector3::new(0., 0., 1.),
                    Vector3::new(0., 0., 1.),
                    Vector3::new(0., 0., 1.),
                ],
            ),
        ];

        assert!(vec_cmp(&triangles, &expected));
    }
}
