use actix_web::{web, HttpResponse};

use rapier3d::math::Vector;

#[derive(serde::Deserialize)]
pub struct FormData {
    size: i32,
    nsubdivs: i32,
}

pub async fn init_heightmap(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

struct Heightmap<T> {
    size: Vector<T>,
    spread: f32,
    spreadRate: f32,
    inner: rapier3d::na::DMatrix<T>,
}
/*
impl<T> Heightmap<T> {
    fn new() -> Self {
        Self {
            size: Vector::new(),
            spread: 0,
            spreadRate: 0,
            inner: Dmatrix::new(),
        }
    }

    pub fn gen_heightmap() -> Self {
        heights = Dmatrix::from_fn()
    }

    fn midpnt_displacement(&self) {}
}
*/
