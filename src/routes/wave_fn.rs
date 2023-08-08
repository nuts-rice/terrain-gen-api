mod wfc {
    use crate::routes::height_map::Heightmap;
    use rand::{Rng, SeedableRng};
    use rand_xorshift::XorShiftRng;
    use std::num::NonZeroU32;
    use wfc::*;
    use wfc_image::ImagePatterns;

    struct HeightmapPattern {
        bottom_left_corner_id: PatternId,
        flower_id: PatternId,
        pixel_coord: Coord,
        pixel_id: PatternId,
    }

    pub fn gen_wfc_heightmap(heightmap: Heightmap) {}
}
