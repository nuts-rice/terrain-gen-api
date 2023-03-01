use rapier3d::na::*;

use rapier3d::prelude::*;

struct Heightmap<T> {
    size: Vector,
    spread: f32,
    spreadRate: f32,
    inner: Dmatrix,
}

impl<T> Heightmap<T> {
    fn new() -> Self {
        Self {
            size: Vector::new(),
            spread: 0,
            spreadRate: 0,
            inner: Dmatrix::new(),
        }
    }

    pub fn init_heightmap() -> Self {
        heights = Dmatrix::from_fn()
    }

    fn midpnt_displacement(&self) {}
}
