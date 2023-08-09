mod wfc {
    use crate::routes::height_map::Heightmap;
    
    
    
    use wfc::*;
    

    struct HeightmapPattern {
        bottom_left_corner_id: PatternId,
        flower_id: PatternId,
        pixel_coord: Coord,
        pixel_id: PatternId,
    }

    pub fn gen_wfc_heightmap(_heightmap: Heightmap) {}
}
