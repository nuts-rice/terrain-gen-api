use rapier3d::na::*;

use rapier3d::prelude::*;

struct Heightmap<T> {
    cols: usize,
    inner: Vec<u32>,
}
