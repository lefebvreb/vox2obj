use std::collections::HashMap;
use std::path::Path;
use std::{fmt, fs};

use block_mesh::ilattice::glam::{IVec3, UVec2, Vec2, Vec3};
use block_mesh::ndshape::{RuntimeShape, Shape};
use block_mesh::{
    GreedyQuadsBuffer, MergeVoxel, Voxel as VoxelTrait, VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG,
};
use dot_vox::Model;

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Voxel {
    pub index: u8,
    pub visibility: VoxelVisibility,
}

impl Voxel {
    const EMPTY: Self = Self {
        index: 0,
        visibility: VoxelVisibility::Empty,
    };
}

impl VoxelTrait for Voxel {
    fn get_visibility(&self) -> VoxelVisibility {
        self.visibility
    }
}

impl MergeVoxel for Voxel {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}

#[derive(Debug)]
struct CubeRepr {
    voxels: Box<[Voxel]>,
}

impl CubeRepr {
    fn new(shape: &RuntimeShape<u32, 3>, vox: &Model) -> Self {
        let mut voxels = vec![Voxel::EMPTY; shape.usize()].into_boxed_slice();

        for v in &vox.voxels {
            let pos = [v.x, v.z, v.y].map(|a| u32::from(a) + 1);
            voxels[shape.linearize(pos) as usize] = Voxel {
                index: v.i.wrapping_add(1),
                visibility: VoxelVisibility::Opaque,
            };
        }

        Self { voxels }
    }
}

#[derive(Debug)]
pub struct Obj {
    offset: Vec2,
    // Vertices.
    v: Vec<Vec3>,
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
    pub fn new(vox: &Model) -> Self {
        let shape =
            RuntimeShape::<u32, 3>::new([vox.size.x, vox.size.z, vox.size.y].map(|a| a + 2));
        let cube = CubeRepr::new(&shape, vox);

        let mut quads_buffer = GreedyQuadsBuffer::new(shape.usize());

        block_mesh::greedy_quads(
            &cube.voxels,
            &shape,
            [0; 3],
            shape.as_array().map(|x| x - 1),
            &RIGHT_HANDED_Y_UP_CONFIG.faces,
            &mut quads_buffer,
        );

        let mut this = Obj {
            offset: UVec2::new(vox.size.x, vox.size.y).as_vec2() * 0.5,
            v: vec![],
            v_map: HashMap::new(),
            vt: vec![],
            vt_map: HashMap::new(),
            vn: vec![],
            vn_map: HashMap::new(),
            q: vec![],
        };

        for (group, face) in quads_buffer
            .quads
            .groups
            .iter()
            .zip(RIGHT_HANDED_Y_UP_CONFIG.faces.as_ref())
        {
            for quad in group.iter() {
                let vertices = face
                    .quad_mesh_positions(quad, 1.0)
                    .map(|v| IVec3::from(v.map(|x| x.round() as i32 - 1)));
                let indices = face.quad_mesh_indices(0);
                let palette_index = cube.voxels[shape.linearize(quad.minimum) as usize].index;
                let normal = face.signed_normal();

                this.push_quad(Quad {
                    vertices,
                    indices,
                    palette_index,
                    normal,
                });
            }
        }

        this
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

    fn v_idx(&mut self, v: IVec3) -> usize {
        *self.v_map.entry(v).or_insert_with(|| {
            let mut v = v.as_vec3();
            v.x -= self.offset.x;
            v.z -= self.offset.y;
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
            writeln!(fmt, "v {x:.1} {y:.1} {z:.1}")?;
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
