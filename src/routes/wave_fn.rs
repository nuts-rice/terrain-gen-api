pub mod wfc {
    use crate::routes::height_map::Heightmap;

    use coord_2d::{Coord, Size};
    use rand::{Rng, SeedableRng};
    use std::num::NonZeroU32;

    use wfc::*;
    use wfc_image::retry;
    use wfc_image::*;
    struct HeightmapPattern {
        bottom_left_corner_id: PatternId,
        flower_id: PatternId,
        pixel_coord: Coord,
        pixel_id: PatternId,
    }

    pub fn gen_wfc_heightmap(_heightmap: Heightmap) {}

    pub fn gen_wfc_image(img: image::DynamicImage, output_path: &str) {
        let output_size = Size::new(64, 64);
        let pattern_size = NonZeroU32::new(3).unwrap();
        let wfc_img = wfc_image::generate_image(
            &img,
            pattern_size,
            output_size,
            &[Orientation::Original],
            WrapXY,
            ForbidNothing,
            retry::NumTimes(10),
        )
        .expect("Too many contradictions");
        wfc_img.save(output_path).expect("failed to save wfc image");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wfc_img_test() {
        let img_path: &str = "../../heightmap.png";
        let _img: image::DynamicImage = ImageReader::open(img_path)?.decode()?;
    }
}
