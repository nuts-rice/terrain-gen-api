use validator::{Validate, ValidationError};

pub struct NewHeightmap {
    pub size: HeightmapSize,
    pub spread_rate: HeightmapSpreadRate,
}

#[derive(Debug, Validate)]
pub struct HeightmapSize {
    #[validate(range(min = 3, max = 257))]
    size: i32,
}

impl HeightmapSize {
    pub fn parse(_u: &i32) -> Result<HeightmapSize, ValidationError> {
        todo!()
    }
    // match  u{
    // Ok(_) => Ok(u)
    // Err(e) => return e;
    // };

    //     // let is_empty_or_whitespace = u.trim().is_empty();
    //     // let is_too_long = u.graphemes(true).count() > 256;
    //     // let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    //     // let contains_forbidden_characters = u.chars().any(|g| forbidden_characters.contains(&g));
    //     match u.validate() {
    //         Ok(_) => (),
    //         Err(e) => return e,
    //         // Err(_) => Err(format!("error when parsing heightmap size")),
    //     };
    //     // if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
    //     //     Err(format!("{} is not a valid heightmap size.", u))
    //     // } else {
    //     //     Ok(Self(u))
    //     // }
    // }
    // }
}
impl AsRef<i32> for HeightmapSize {
    fn as_ref(&self) -> &i32 {
        &self.size
    }
}

#[derive(Debug, Validate)]
pub struct HeightmapSpreadRate {
    #[validate(range(min = 0.01, max = 1.0))]
    rate: f64,
}
impl HeightmapSpreadRate {
    pub fn parse(_u: &f64) -> Result<HeightmapSpreadRate, String> {
        todo!()
    }
    //     // let is_empty_or_whitespace = u.trim().is_empty();
    //     // let is_too_long = u.graphemes(true).count() > 256;
    //     // let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    //     // let contains_forbidden_characters = u.chars().any(|g| forbidden_characters.contains(&g));
    //     match u.validate() {
    //         Ok(_) => (),
    //         Err(e) => return e,
    //     };
    //     // if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
    //     //     Err(format!("{} is not a valid heightmap size.", u))
    //     // } else {
    //     //     Ok(Self(u))
    //     // }
    // }
    // }
}
impl AsRef<f64> for HeightmapSpreadRate {
    fn as_ref(&self) -> &f64 {
        &self.rate
    }
}

impl NewHeightmap {
    pub fn create_new_heightmap(
        size: HeightmapSize,
        spread_rate: HeightmapSpreadRate,
    ) -> NewHeightmap {
        NewHeightmap { size, spread_rate }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok};

    #[test]
    fn size_greater_than_256_test() {
        let size = "6".repeat(256);
        assert_ok!(HeightmapSize::parse(&size));
    }

    #[test]
    fn size_greater_than_257_test() {
        let size = "6".repeat(257);
        assert_err!(HeightmapSize::parse(&size));
    }
}
