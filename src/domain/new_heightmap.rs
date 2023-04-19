use unicode_segmentation::UnicodeSegmentation;

pub struct NewHeightmap {
    pub size: HeightmapSize,
    pub spread_rate: HeightmapSpreadRate,
}

#[derive(Debug)]
pub struct HeightmapSize(usize);
impl HeightmapSize {
    pub fn parse(u: &str) -> Result<HeightmapSize, String> {
        let is_empty_or_whitespace = u.trim().is_empty();
        let is_too_long = u.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = u.chars().any(|g| forbidden_characters.contains(&g));
        let value = match u.parse::<usize>() {
            Ok(value) => value,
            Err(_) => todo!(),
        };
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid heightmap size.", u))
        } else {
            Ok(Self(value))
        }
    }
}

impl AsRef<usize> for HeightmapSize {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

#[derive(Debug)]
pub struct HeightmapSpreadRate(f64);
impl HeightmapSpreadRate {
    pub fn parse(u: &str) -> Result<HeightmapSpreadRate, String> {
        let is_empty_or_whitespace = u.trim().is_empty();
        let is_too_long = u.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = u.chars().any(|g| forbidden_characters.contains(&g));
        let value = match u.parse::<f64>() {
            Ok(value) => value,
            Err(_) => todo!(),
        };
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid heightmap size.", u))
        } else {
            Ok(Self(value))
        }
    }
}

impl AsRef<f64> for HeightmapSpreadRate {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl NewHeightmap {
    pub fn create_new_heightmap(size: HeightmapSize, spread_rate: HeightmapSpreadRate) -> NewHeightmap {
        NewHeightmap {size, spread_rate}
    }
}
