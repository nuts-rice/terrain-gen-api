use actix_web::{web, HttpResponse};

use anyhow::Error;
use ndarray::Array;
use ndarray::{Array2, Axis};
use rand::Rng;
//use rapier3d::na::{Vector3, SquareMatrix};
//use rapier3d::parry::utils::self.inner2;

#[derive(serde::Deserialize)]
pub struct FormData {
    size: usize,
    spread_rate: f64,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    name = "generating a new heightmap",
    skip(_form),
    fields(
            heightmap_size = %_form.size,
            heightmap_spread_rate = %_form.spread_rate,
        )
)]

pub async fn serve_heightmap(_form: web::Form<FormData>) -> HttpResponse {
    //TODO: parse this in and inner
    let mut new_heightmap = Heightmap {
        size: _form.size,
        spread_rate: _form.spread_rate,
        inner: Array::from_elem((_form.size + 1, _form.size + 1), 0.0),
    };
    match Heightmap::generate_heightmap(&mut new_heightmap).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
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
