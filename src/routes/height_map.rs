use actix_web::{web, HttpResponse};

use anyhow::Error;
use rapier3d::na::Vector3;
use rapier3d::prelude::*;

#[derive(serde::Deserialize)]
pub struct FormData {
    size: i32,
    nsubdivs: i32,
    spreadRate: f32,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    name = "generating a new heightmap",
    skip(_form),
    fields(
            heightmap_size = %_form.size,
            heightmap_nsubdivs = %_form.nsubdivs,
            heightmap_spreadrate = %_form.spreadRate,
        )
)]

pub async fn serve_heightmap(_form: web::Form<FormData>) -> HttpResponse {
    //TODO: parse this in and inner
    let new_heightmap = Heightmap {
        size: Vector::new(0, 0, 0),
        spread: 0.0,
        spreadRate: 0.0,
        //        inner: rapier3d::na::dmatrix![0, 0, 0,
        //        0, 0, 0,
        //        0, 0, 0],
    };
    match Heightmap::generate_heightmap(&new_heightmap).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

struct Heightmap {
    size: Vector3<i32>,
    spread: f32,
    spreadRate: f32,
    //    inner: rapier3d::na::DMatrix<T>,
}

impl Heightmap {
    fn new() -> Self {
        Self {
            //TODO: figure parsing size and inner
            size: Vector3::new(0, 0, 0),
            spread: 0.0,
            spreadRate: 0.0,
            //          inner,
        }
    }

    pub async fn generate_heightmap(&self) -> Result<(), Error> {
        let _ = &Self::midpnt_displacement().await.map_err(|e| {
            tracing::error!("failed to midpoint displacement:  {:?}", e);
            e
        })?;
        Ok(())
    }

    pub async fn midpnt_displacement() -> Result<(), Error> {
        Ok(())
    }
}
