use actix_web::{http::header::ContentType, post, web, Error, HttpResponse, Result};

use cgmath::Vector2;

use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_xorshift::XorShiftRng;
use rapier3d::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

//TODO: THIS BREAKS THINGS
//use bevy_rapier3d::prelude::*;
//use rapier3d::na::{Vector3, SquareMatrix};
//use rapier3d::parry::utils::self.inner2;

//TODO: use seed for gen
use image::{DynamicImage, ImageBuffer, Pixel, Rgb, Rgba};

#[derive(serde::Deserialize)]
pub struct FormData {
    exponent: i32,
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
pub const WHITE: [u8; 3] = [255, 255, 255];

pub const BLACK: [u8; 3] = [0, 0, 0];
pub const RED: [u8; 3] = [255, 0, 0];
pub const GREEN: [u8; 3] = [0, 255, 0];
pub const BLUE: [u8; 3] = [0, 0, 255];
// Removed TRANSPARENT as transparency is not handled in RGB model.

pub const ORANGE: [u8; 3] = [255, 69, 0];

//TODO: compute color gradient
#[derive(Debug)]
pub struct Heightmap {
    size: i32,
    spread_rate: f32,
    inner: Vector2<usize>,
    heights: Vec<Vec<f32>>,
    seed: u64,
    colors: Vec<[u8; 3]>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeightsResponse {
    heights: Vec<f32>,
}

impl Heightmap {
    pub async fn new(exponent: i32, _spread_rate: f32, _seed: u64) -> Result<Heightmap, Error> {
        let _size = 2_i32.pow(exponent.try_into().unwrap()) as usize;
        tracing::info!(
            "creating heightmap of {} by {} with spread rate of {}, Seed: {}",
            exponent,
            exponent,
            _spread_rate,
            _seed
        );
        let _heights = vec![vec![0.0; _size]; _size];
        let _colors = vec![WHITE];
        Ok(Heightmap {
            size: exponent,
            spread_rate: _spread_rate,
            inner: Vector2 { x: _size, y: _size },
            heights: _heights,
            seed: _seed,
            colors: _colors,
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
        let mut resolution = 2_f32.powf(self.size as f32) - 1.0;
        let mut _rng: ChaCha20Rng = rand::SeedableRng::seed_from_u64(self.seed);
        let _step = self.size;
        while resolution > 1.0 {
            let curr_step: usize = (resolution / 2.0).round() as usize;
            let rng_val = _rng.gen_range(0.0..1.0);
            self.heights
                .par_iter_mut()
                .enumerate()
                .for_each(|(i, row)| {
                    if Self::is_in_range(i, curr_step, self.inner.x) {
                        row.par_iter_mut().enumerate().for_each(|(j, height)| {
                            if Self::is_in_range(j, curr_step, self.inner.y) {
                                let _square_average = (*height + resolution / 2.0) / 4.0;
                                let displacement = (rng_val - 0.5) * resolution * self.spread_rate;

                                *height = _square_average + displacement;
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
                        });
                    }
                });
            resolution /= 2.0;
        }
        //TODO: need to shuffle this randomly
        let mut shuffled_indices: Vec<_> = (0..self.heights.len()).collect();
        shuffled_indices.shuffle(&mut _rng);
        let shuffled_heights: Vec<_> = shuffled_indices
            .par_iter_mut()
            .map(|&mut i| self.heights[i].clone())
            .collect();
        self.heights = shuffled_heights;
        Ok(())
    }

    fn is_in_range(idx: usize, curr_step: usize, limit: usize) -> bool {
        idx >= curr_step / 2 && idx < limit - 1 && idx % curr_step == 0
    }

    pub async fn render(&self, file_path: &str) -> Result<(), Error> {
        let mut _rng = rand::thread_rng();
        let mut img = ImageBuffer::new(self.inner.x as u32, self.inner.y as u32);
        let mut rgba_grad = [0, 0, 0, 0];

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let height = self.heights[x as usize][y as usize];
            let gray_value = (height * 255.0) as u8;
            let rgba_grad_index = _rng.gen_range(0..4);
            rgba_grad[rgba_grad_index] = gray_value;
            // tracing::debug!("gray val: {}", gray_value);
            *pixel = Rgba(rgba_grad);
            let _data = pixel.0;
            // tracing::debug!("color vals: {:?}", data);
        }
        img.save(file_path).expect("error in rendering");
        Ok(())
    }

    //heights match to constants, using invariant coloring rules
    pub async fn color_terrain(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn render_2d_test(&self, file_path: &str) -> Result<DynamicImage, Error> {
        let mut img = ImageBuffer::new(self.inner.x as u32, self.inner.y as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let height = self.heights[x as usize][y as usize];
            let gray_value = (height * 255.0) as u8;
            *pixel = Rgba([gray_value, gray_value, gray_value, gray_value]);
        }
        img.save(file_path).expect("error in rendering");
        Ok((DynamicImage::from(img)))
    }

    //TODO: Ok try to use Arc<[T]> here?
    pub async fn render_3d_test(&self) -> Result<HeightsResponse, Error> {
        //uh needs physics state here
        let _ground_size = Vector::new(100.0, 1.0, 100.0);
        let flat_land: Vec<f32> =
            Vec::from(self.heights.iter().flatten().copied().collect::<Vec<_>>());
        // tracing::debug!("heights: {:?}", flat_land.iter().map(|h| h.to_string()).collect::<Vec<_>>().join(", "));
        // let _heights = DMatrix::from_vec(self.size as usize, self.size as usize, flat_land.to_vec());
        // let _heightfield = ColliderBuilder::heightfield(_heights, ground_size).build();

        // let _debug_render = DebugRenderPipeline::default();

        Ok(HeightsResponse {
            heights: flat_land.clone(),
        })
    }

    //Testing purposes
    // pub async fn render_3d_arc(&self) -> Result<(), Error> {
    //     let ground_size = Vector::new(100.0, 1.0, 100.0);
    //     let flat_land: Arc<f32> = self.heights.iter().flatten().copied();
    //     let _heights = DMatrix::from_vec(self.size as usize, self.size as usize, flat_land);
    //     let _heightfield = ColliderBuilder::heightfield(_heights, ground_size).build();

    //     let _debug_render = DebugRenderPipeline::default();

    //     Ok(())
    // }
}

impl fmt::Display for Heightmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flat_land: Arc<[f32]> =
            Arc::from(self.heights.iter().flatten().copied().collect::<Vec<f32>>());
        write!(
            f,
            "heights: {:?}",
            flat_land
                .iter()
                .map(|h| h.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[post("/new_heightmap/{exponent}/{spread_rate}")]
pub async fn new_heightmap(path: web::Path<(i32, f32, u64)>) -> Result<HttpResponse, Error> {
    let (exponent, spread_rate, seed) = path.into_inner();
    tracing::info!(
        "creating heightmap of {} by {} with spread rate of {}, Seed: {}",
        exponent,
        exponent,
        spread_rate,
        seed
    );
    let mut heightmap = Heightmap::new(exponent, spread_rate, seed).await.unwrap();
    heightmap
        .midpnt_displacement()
        .await
        .map_err(|e| {
            actix_web::error::ErrorImATeapot(e);
        })
        .expect("error in midpnt displacement");
    let img = heightmap.render("heightmap.png").await.unwrap();
    // map_err(|e| {
    // tracing::error!("failed to midpoint displacement:  {:?}", e);
    // e        })?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::png())
        .body(img))
}
