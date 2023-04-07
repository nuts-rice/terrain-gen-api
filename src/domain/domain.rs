use unicode_segmentation::UnicodeSegmentation;
use validator::Validate;

pub struct NewHeightmap {
    pub size: HeightmapSize,
    pub spread_rate: HeightmapSpreadRate,
}

#[derive(Debug)]
pub struct HeightmapSize(usize);
impl HeightmapSize {
    pub fn parse(u: usize) -> Result<HeightmapSize, String> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct HeightmapSpreadRate(f64);
impl HeightmapSpreadRate {
    pub fn parse(f: f64) -> Result<HeightmapSpreadRate, String> {
        unimplemented!()
    }
}
