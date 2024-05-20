# Vox2Obj

Convert MagicaVoxel's `.vox` files to optimized mesh `.obj` files, from the command line.

## Supported features

* Export scene trees to multiple `.obj` files, one per frame per model.
* UV, albedo, metallic, roughness, emission.

## Unsupported features

* Transforms (translation + rotation).
* Transparency

## Future ideas

One may be able to use the [spade](https://crates.io/crates/spade) crate to perform constrained Delaunay triangulation on the polygonal faces of the model, to get an optimal (in the vertex count) meshing of the model.
