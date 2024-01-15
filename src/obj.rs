use std::fs::File;
use std::io;
use crate::shapes::Triangle;
use io::{Error, Result};
use std::io::{BufRead, ErrorKind};
use crate::material::Material;
use crate::vector::Vec3;

pub fn obj_to_triangles(file_path: &str, pos: Vec3, material: Material) -> Result<Vec<Triangle>> {
    let file = File::open(file_path)?;
    let lines= io::BufReader::new(file).lines();

    let mut vertices = Vec::new();
    let mut triangles = Vec::new();

    for line in lines {
        let line = line?;
        let mut parts = line.split_whitespace();
        let line_type = parts.next();
        let line_type = match line_type {
            None => continue,
            Some(str) => str
        };
        match line_type {
            "v" => {
                // Vertex

                let x: f64 = parts.next().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Not enough elements"))?.parse().map_err(|err| Error::new(ErrorKind::InvalidData, "Invalid numbers"))?;
                let y: f64 = parts.next().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Not enough elements"))?.parse().map_err(|err| Error::new(ErrorKind::InvalidData, "Invalid numbers"))?;
                let z: f64 = parts.next().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Not enough elements"))?.parse().map_err(|err| Error::new(ErrorKind::InvalidData, "Invalid numbers"))?;

                vertices.push(Vec3::new(x, y, z));
            }
            "f" => {
                // Polygon

                let v0_index: usize = parts.next().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Not enough elements"))?.split("/").next().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Not enough elements"))?.parse().map_err(|err| Error::new(ErrorKind::InvalidData, "Invalid numbers"))?;
                let v1_index: usize = parts.next().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Not enough elements"))?.split("/").next().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Not enough elements"))?.parse().map_err(|err| Error::new(ErrorKind::InvalidData, "Invalid numbers"))?;
                let v2_index: usize = parts.next().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Not enough elements"))?.split("/").next().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Not enough elements"))?.parse().map_err(|err| Error::new(ErrorKind::InvalidData, "Invalid numbers"))?;

                let v0 = vertices.get(v0_index - 1).ok_or_else(|| Error::new(ErrorKind::InvalidData, "Vertex not found"))?;
                let v1 = vertices.get(v1_index - 1).ok_or_else(|| Error::new(ErrorKind::InvalidData, "Vertex not found"))?;
                let v2 = vertices.get(v2_index - 1).ok_or_else(|| Error::new(ErrorKind::InvalidData, "Vertex not found"))?;

                let triangle = Triangle::new(*v0 + pos, *v1 + pos, *v2 + pos, material.clone());
                println!("triangle = {:?}", triangle);
                triangles.push(triangle);

            }
            _default => {}
        }
    }

    Ok(triangles)
}