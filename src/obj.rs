use std::fmt;

use crate::math::{Face, FaceVertex, Vec3};

#[derive(Default, Debug)]
pub struct Obj {
    /// Texture coordinates.
    pub vt: Vec<[f64; 2]>,
    /// Vertices.
    pub v: Vec<Vec3>,
    /// Faces.
    pub f: Vec<Face>,
}

impl Obj {
    pub fn push_vt(&mut self, i: u8) -> usize {
        todo!()
    }

    pub fn push_v(&mut self, v: Vec3) -> usize {
        todo!()
    }

    pub fn push_face(&mut self, f: Face) -> usize {
        todo!()
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write normals.
        writeln!(fmt, "vn -1 0 0\nvn 1 0 0\nvn 0 0 1\nvn 0 0 -1\nvn 0 -1 0\nvn 0 1 0")?;

        // Write texture coordinates.
        for [x, y] in &self.vt {
            writeln!(fmt, "vt {x:.6} {y:.6}")?;
        }

        // Write vertices.
        for Vec3 { x, y, z } in &self.v {
            writeln!(fmt, "v {x} {y} {z}")?;
        }

        // Write vertices.
        for [
            FaceVertex { v: v1, vt: vt1, vn: vn1 },
            FaceVertex { v: v2, vt: vt2, vn: vn2 },
            FaceVertex { v: v3, vt: vt3, vn: vn3 },
        ] in &self.f {
            writeln!(fmt, "f {v1}/{vt1}/{vn1} {v2}/{vt2}/{vn2} {v3}/{vt3}/{vn3}")?;
        }

        Ok(())
    }
}
