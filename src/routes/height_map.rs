use actix_web::{web, HttpResponse};

use anyhow::Error;
use ndarray::Array;
use ndarray::{Array2, Axis};
use rand::Rng;
//use rapier3d::na::{Vector3, SquareMatrix};
//use rapier3d::parry::utils::self.inner2;

use crate::domain::{HeightmapSize, HeightmapSpreadRate, NewHeightmap};
use std::convert::TryFrom;

#[derive(serde::Deserialize)]
pub struct FormData {
    size: String,
    spread_rate: String,
}

impl TryFrom<FormData> for NewHeightmap {
    type Error = String;

    fn try_from(value: FormData) -> Result<NewHeightmap, Self::Error> {
        let size = HeightmapSize::parse(&value.size)?;
        let spread_rate = HeightmapSpreadRate::parse(&value.spread_rate)?;
        Ok(NewHeightmap { size, spread_rate })
    }
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    name = "generating a new heightmap",
    skip(form),
    fields(
            heightmap_size = %form.size,
            heightmap_spread_rate = %form.spread_rate,
        )
)]

pub async fn serve_heightmap(form: web::Form<FormData>) -> HttpResponse {
    //TODO: parse this in and inner
    let size = match HeightmapSize::parse(&form.0.size) {
        Ok(size) => size,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let spread_rate = match HeightmapSpreadRate::parse(&form.0.spread_rate) {
        Ok(spread_rate) => spread_rate,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let _new_heightmap = NewHeightmap::create_new_heightmap(size, spread_rate);
    todo!()
    // match Heightmap::generate_heightmap(&mut _new_heightmap).await {
    //         Ok(_) => HttpResponse::Ok().finish(),
    //         Err(_) => HttpResponse::InternalServerError().finish(),
    //     }
}

#[derive(Debug)]
struct Heightmap {
    size: usize,
    spread_rate: f64,
    inner: Array2<f64>,
}

impl Heightmap {
    fn new(size: usize, _spread_rate: f64) -> Self {
        Self {
            //TODO: figure parsing size and inner
            size: 9,
            spread_rate: 0.0,
            inner: Array::from_elem((size + 1, size + 1), 0.0),
        }
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
        let mut step = self.size;
        while step > 1 {
            for i in (step / 2..self.inner.len_of(Axis(0)) - 1).step_by(step) {
                for j in (step / 2..self.inner.len_of(Axis(1)) - 1).step_by(step) {
                    let _square_average = (self.inner[[i - step / 2, j - step / 2]]
                        + self.inner[[i - step / 2, j + step / 2]]
                        + self.inner[[i + step / 2, j - step / 2]]
                        + self.inner[[i + step / 2, j + step / 2]])
                        / 4.0;
                    let displacement =
                        (_rng.gen_range(0.0..1.0) - 0.5) * step as f64 * self.spread_rate;
                    self.inner[[i, j]] = _square_average + displacement;

                    if j > step / 2 {
                        let diamond_average = (self.inner[[i - step / 2, j]]
                            + self.inner[[i + step / 2, j]]
                            + self.inner[[i, j - step / 2]]
                            + self.inner[[i, j + step / 2]])
                            / 4.0;
                        let displacement =
                            (_rng.gen_range(0.0..1.0) - 0.5) * step as f64 * self.spread_rate;
                        self.inner[[i, j - step / 2]] = diamond_average + displacement;
                    }

                    if i > step / 2 {
                        let diamond_average = (self.inner[[i - step / 2, j]]
                            + self.inner[[i + step / 2, j]]
                            + self.inner[[i, j - step / 2]]
                            + self.inner[[i, j + step / 2]])
                            / 4.0;
                        let displacement =
                            (_rng.gen_range(0.0..1.0) - 0.5) * step as f64 * self.spread_rate;
                        self.inner[[i - step / 2, j]] = diamond_average + displacement;
                    }
                }
            }
            step /= 2;
        }
        Ok(())
    }
}
