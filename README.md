# Vox2Obj

Convert [MagicaVoxel](https://ephtracy.github.io/)'s `.vox` files to optimized mesh `.obj` files, from the command line.

**Supported features**:
* Export scene trees to multiple `.obj` files, one per frame per model.
* Export palette textures: albedo, metalness, roughness, emission.

**Unsupported features**:
* Transforms (translation + rotation).
* Transparency.

Exported models have a scale of 1 block = 1 unit.

## Installation and Usage

Install [`cargo`](https://crates.io/), then type in your favorite shell:

```sh
cargo install --git https://github.com/lefebvreb/vox2obj
```

You can then execute `vox2obj` like so:

```sh
vox2obj --help
```

## Future ideas

One may be able to use the [spade](https://crates.io/crates/spade) crate to perform constrained Delaunay triangulation on the polygonal faces of the model, to get an optimal (in the vertex count) meshing of the model.
