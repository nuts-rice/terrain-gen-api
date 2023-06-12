use actix_web::{http::header::ContentType, post, web, HttpResponse, Result};

use anyhow::{Error, Ok};

use rand::Rng;

use cgmath::Vector2;
//use rapier3d::na::{Vector3, SquareMatrix};
//use rapier3d::parry::utils::self.inner2;

use image::{ImageBuffer, Rgb};

#[derive(serde::Deserialize)]
pub struct FormData {
    size: i32,
    spread_rate: f64,
}

// impl TryFrom<FormData> for NewHeightmap {
//     type Error = String;

//     fn try_from(value: FormData) -> Result<NewHeightmap, String> {
//         let size = HeightmapSize::parse(&value.size).unwrap();

//         let spread_rate = HeightmapSpreadRate::parse(&value.spread_rate).unwrap();
//         Ok(Self { size, spread_rate })
//     }
// }

// #[allow(clippy::async_yields_async)]
// #[tracing::instrument(
//     name = "generating a new heightmap",
//     skip(form),
//     fields(
//             heightmap_size = %form.size,
//             heightmap_spread_rate = %form.spread_rate,
//         )
// )]

//pub async fn serve_heightmap(form: web::Form<FormData>) -> HttpResponse {
//    //TODO: parse this in and inner
//    let _new_heightmap: NewHeightmap = match form.0.try_into() {
//        Ok(form) => form,
//        Err(_) => return HttpResponse::BadRequest().finish(),
//    };

// Ok(HeightmapSize) => size,
// Err(_) => return HttpResponse::BadRequest().finish(),
// };
// let spread_rate = match HeightmapSpreadRate::parse(&form.0.spread_rate).expect("spread rate validation failed") {
// Ok(_) => spread_rate,
// Err(_) => return HttpResponse::BadRequest().finish(),

// match _new_heightmap(&mut _new_heightmap).await {
//          Ok(_) => HttpResponse::Ok().finish(),
//          Err(_) => HttpResponse::InternalServerError().finish(),
//      }
// HttpResponse::Ok().finish()
// }

// #[derive(Debug)]
// pub struct ColorGradient {
//     color_vals: Vec<[u8; 3]>,
// }
// impl ColorGradient {
//     pub const WHITE: Self = Self {
//         color_vals: vec![
//             [255, 255, 255],
//         ],
//     };

//     pub const BLACK: Self = Self {
//         color_vals: vec![
//             [0, 0, 0],
//         ],
//     };

//     pub const RED: Self = Self {
//         color_vals: vec![
//             [255, 0, 0],
//         ],
//     };

//     pub const GREEN: Self = Self {
//         color_vals: vec![
//             [0, 255, 0],
//         ],
//     };

//     pub const BLUE: Self = Self {
//         color_vals: vec![
//             [0, 0, 255],
//         ],
//     };

//     // Removed TRANSPARENT as transparency is not handled in RGB model.

//     pub const ORANGE: Self = Self {
//         color_vals: vec![
//             [255, 69, 0],
//         ],
//     };
//}

#[derive(Debug)]
pub struct Heightmap {
    size: i32,
    spread_rate: f32,
    inner: Vector2<usize>,
    heights: Vec<Vec<f32>>,
}

impl Heightmap {
    pub async fn new(exponent: i32, _spread_rate: f32) -> Result<Heightmap, Error> {
        let _size = 2_i32.pow(exponent.try_into().unwrap()) as usize;
        tracing::info!(
            "creating heightmap of {} by {} with spread rate of {}",
            exponent,
            exponent,
            _spread_rate
        );
        let _heights = vec![vec![0.0; _size]; _size];
        Ok(Heightmap {
            size: exponent,
            spread_rate: _spread_rate,
            inner: Vector2 { x: _size, y: _size },
            heights: _heights,
        })
    }

    #[tracing::instrument(name = "randomizing heightmap")]
    pub async fn generate_heightmap(&mut self) -> Result<(), Error> {
        self.midpnt_displacement().await.map_err(|e| {
            tracing::error!("failed to midpoint displacement:  {:?}", e);
            e
        })?;
        Ok(())
    }

    pub async fn midpnt_displacement(&mut self) -> Result<(), Error> {
        let mut _rng = rand::thread_rng();
        let mut resolution = 2_f32.powf(self.size as f32) - 1.0;
        let _step = self.size;
        while resolution > 1.0 {
            let curr_step: usize = (resolution / 2.0).round() as usize;
            for i in (curr_step / 2..self.inner.x - 1).step_by(curr_step) {
                for j in (curr_step / 2..self.inner.y - 1).step_by(curr_step) {
                    let _square_average = (self.heights[i][j] + resolution / 2.0) / 4.0;
                    let displacement =
                        (_rng.gen_range(0.0..1.0) - 0.5) * resolution * self.spread_rate;
                    self.heights[i][j] = _square_average + displacement;
                    tracing::debug!("Height : {}", self.heights[i][j]);
                    // if j > step / 2 {
                    //     let diamond_average = (self.inner[[i - step / 2, j]]
                    //         + self.inner[[i + step / 2, j]]
                    //         + self.inner[[i, j - step / 2]]
                    //         + self.inner[[i, j + step / 2]])
                    //         / 4.0;
                    //     let displacement =
                    //         (_rng.gen_range(0.0..1.0) - 0.5) * step as f64 * self.spread_rate;
                    //     self.inner[[i, j - step / 2]] = diamond_average + displacement;
                    // }

                    // if i > step / 2 {
                    //     let diamond_average = (self.inner[[i - step / 2, j]]
                    //         + self.inner[[i + step / 2, j]]
                    //         + self.inner[[i, j - step / 2]]
                    //         + self.inner[[i, j + step / 2]])
                    //         / 4.0;
                    //     let displacement =
                    //         (_rng.gen_range(1.0..1.0) - 0.5) * step as f64 * self.spread_rate;
                    //     self.inner[[i - step / 2, j]] = diamond_average + displacement;
                    // }
                }
            }
            resolution /= 2.0;
        }
        Ok(())
    }

    pub async fn render(&self, file_path: &str) -> Result<(), Error> {
        let mut _rng = rand::thread_rng();
        let mut img = ImageBuffer::new(self.inner.x as u32, self.inner.y as u32);
        let mut color_grad = [0, 0, 0];

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let height = self.heights[x as usize][y as usize];
            let gray_value = (height * 255.0) as u8;
            let color_grad_index = _rng.gen_range(0..3);
            color_grad[color_grad_index] = gray_value;
            tracing::debug!("gray val: {}", gray_value);
            *pixel = Rgb(color_grad);
            let data = pixel.0;
            tracing::debug!("color vals: {:?}", data);
        }
        img.save(file_path)?;
        Ok(())
    }
}
#[post("/new_heightmap/{exponent}/{spread_rate}")]
async fn new_heightmap(path: web::Path<(i32, f32)>) -> HttpResponse {
    let (exponent, spread_rate) = path.into_inner();
    tracing::info!(
        "creating heightmap of {} by {} with spread rate of {}",
        exponent,
        exponent,
        spread_rate
    );
    let mut heightmap = Heightmap::new(exponent, spread_rate).await.unwrap();
    heightmap.midpnt_displacement().await.map_err(|e| {
        actix_web::error::ErrorImATeapot(e);
    });
    let img = heightmap.render("heightmap.png").await.unwrap();
    // map_err(|e| {
    // tracing::error!("failed to midpoint displacement:  {:?}", e);
    // e        })?;

    HttpResponse::Ok()
        .content_type(ContentType::png())
        .body(img)
}
