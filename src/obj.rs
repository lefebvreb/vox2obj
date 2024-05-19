use std::collections::HashMap;
use std::path::Path;
use std::{fmt, fs};

use block_mesh::ilattice::glam::IVec3;

use crate::error::Result;

#[derive(Debug)]
pub struct Quad {
    pub vertices: [IVec3; 4],
    pub indices: [u32; 6],
    pub palette_index: u8,
    pub normal: IVec3,
}

#[derive(Debug)]
struct QuadIndices {
    v: [usize; 6],
    vt: usize,
    vn: usize,
}

#[derive(Default, Debug)]
pub struct Obj {
    // Vertices.
    v: Vec<IVec3>,
    v_map: HashMap<IVec3, usize>,
    // Texture coordinates.
    vt: Vec<f32>,
    vt_map: HashMap<u8, usize>,
    // Normals.
    vn: Vec<IVec3>,
    vn_map: HashMap<IVec3, usize>,
    // Quads.
    q: Vec<QuadIndices>,
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
            self.vt.push((f32::from(i) - 0.5) / 256.0);
            self.vt.len()
        })
    }

    fn vn_idx(&mut self, vn: IVec3) -> usize {
        *self.vn_map.entry(vn).or_insert_with(|| {
            self.vn.push(vn);
            self.vn.len()
        })
    }

    pub fn push_quad(&mut self, quad: Quad) {
        let v = quad
            .indices
            .map(|i| quad.vertices[i as usize])
            .map(|vertex| self.v_idx(vertex));
        let vt = self.vt_idx(quad.palette_index);
        let vn = self.vn_idx(quad.normal);
        self.q.push(QuadIndices { v, vt, vn });
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::write(path.as_ref(), self.to_string())?;
        Ok(())
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
            writeln!(fmt, "vt {vt:.6} 0.5")?;
        }

        // Write normals.
        for vn in &self.vn {
            let [x, y, z] = vn.to_array();
            writeln!(fmt, "vn {x} {y} {z}")?;
        }

        // Write faces.
        for QuadIndices {
            v: [v1, v2, v3, v4, v5, v6],
            vt,
            vn,
        } in &self.q
        {
            writeln!(
                fmt,
                "f {v1}/{VT}/{VN} {v2}/{VT}/{VN} {v3}/{VT}/{VN}\nf {v4}/{VT}/{VN} {v5}/{VT}/{VN} {v6}/{VT}/{VN}",
                VT = vt,
                VN = vn
            )?;
        }

        Ok(())
    }
}
