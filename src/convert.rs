// Ideally, one would use spade (a rust crate) to perform Constrained Delaunay Triangulation
// to get an optimal meshing of the visible faces.

use block_mesh::ilattice::glam::IVec3;
use block_mesh::ndshape::{RuntimeShape, Shape};
use block_mesh::{
    GreedyQuadsBuffer, MergeVoxel, Voxel as VoxelTrait, VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG,
};
use dot_vox::Model;

use crate::obj::{Face, Obj};

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

pub fn convert_model(vox: &Model) -> Obj {
    let shape = RuntimeShape::<u32, 3>::new([vox.size.x, vox.size.z, vox.size.y].map(|a| a + 2));
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

    let mut obj = Obj::default();

    for (group, face) in quads_buffer
        .quads
        .groups
        .iter()
        .zip(RIGHT_HANDED_Y_UP_CONFIG.faces.as_ref())
    {
        for quad in group.iter() {
            let [a, b, c, d] = face
                .quad_mesh_positions(quad, 1.0)
                .map(|v| IVec3::from(v.map(|x| x.round() as i32 - 1)));
            let palette_index = cube.voxels[shape.linearize(quad.minimum) as usize].index;
            let normal = face.signed_normal();

            obj.push_face(Face {
                vertices: [a, b, c],
                palette_index,
                normal,
            });
            obj.push_face(Face {
                vertices: [b, d, c],
                palette_index,
                normal,
            });
        }
    }

    obj
}
