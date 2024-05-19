use std::collections::HashMap;
use std::fmt;

use block_mesh::ilattice::glam::{IVec3, Vec2};

#[derive(Debug)]
pub struct Face {
    pub vertices: [IVec3; 3],
    pub palette_index: u8,
    pub normal: IVec3,
}

#[derive(Debug)]
struct FaceIndices {
    v: [usize; 3],
    vt: usize,
    vn: usize,
}

#[derive(Default, Debug)]
pub struct Obj {
    // Vertices.
    v: Vec<IVec3>,
    v_map: HashMap<IVec3, usize>,
    // Texture coordinates.
    vt: Vec<Vec2>,
    vt_map: HashMap<u8, usize>,
    // Normals.
    vn: Vec<IVec3>,
    vn_map: HashMap<IVec3, usize>,
    // Faces.
    f: Vec<FaceIndices>,
}

impl Obj {
    fn v_idx(&mut self, v: IVec3) -> usize {
        *self.v_map.entry(v).or_insert_with(|| {
            self.v.push(v);
            self.v.len()
        })
    }

    fn vt_idx(&mut self, i: u8) -> usize {
        *self.vt_map.entry(i).or_insert_with(|| {
            self.vt
                .push(Vec2::new((i % 16) as f32 + 0.5, (i / 16) as f32 + 0.5));
            self.vt.len()
        })
    }

    fn vn_idx(&mut self, vn: IVec3) -> usize {
        *self.vn_map.entry(vn).or_insert_with(|| {
            self.vn.push(vn);
            self.vn.len()
        })
    }

    pub fn push_face(&mut self, face: Face) {
        let v = face.vertices.map(|vertex| self.v_idx(vertex));
        let vt = self.vt_idx(face.palette_index);
        let vn = self.vn_idx(face.normal);
        self.f.push(FaceIndices { v, vt, vn });
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write vertices.
        for v in &self.v {
            let [x, y, z] = v.to_array();
            writeln!(fmt, "v {x} {y} {z}")?;
        }

        // Write texture coordinates.
        for vt in &self.vt {
            let [x, y] = vt.to_array();
            writeln!(fmt, "vt {x:.6} {y:.6}")?;
        }

        // Write normals.
        for vn in &self.vn {
            let [x, y, z] = vn.to_array();
            writeln!(fmt, "vn {x} {y} {z}")?;
        }

        for FaceIndices {
            v: [v1, v2, v3],
            vt,
            vn,
        } in &self.f
        {
            writeln!(
                fmt,
                "f {v1}/{VT}/{VN} {v2}/{VT}/{VN} {v3}/{VT}/{VN}",
                VT = vt,
                VN = vn
            )?;
        }

        Ok(())
    }
}
