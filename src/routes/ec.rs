use actix_web::{web, HttpResponse};

pub use core::{
    fmt,
    ops::{Add, Mul, Neg},
    str,
};
use std::fmt::{Display, Formatter};

#[derive(serde::Deserialize)]
pub struct EC {
    a: u64,
    b: u64,
}

pub struct Point {
    x: u64,
    y: u64,
}

impl EC {
    pub const MODULUS: i32 = 2_i32.pow(256) - 2_i32.pow(32) - 977;
    pub const ORDER: i32 = 2_i32.pow(256);

    pub async fn reals_point_mult() -> HttpResponse {
        HttpResponse::Ok().finish()
    }
    pub async fn reals_point_add() -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    pub async fn getInputs(form: web::Form<EC>) -> HttpResponse {
        let _a = form.a;
        let _b = form.b;
        HttpResponse::Ok().finish()
    }

    pub async fn to_repr(self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "order: {}, a: {}, b: {}, p: {}, equation: y^2 = x^3 + 2x + 3",
            EC::ORDER,
            self.a,
            self.b,
            EC::MODULUS
        )
    }

    pub async fn recalculate() {
        todo!()
    }

    pub async fn update() {
        todo!()
    }

    pub async fn redraw() {
        todo!()
    }
}

impl fmt::UpperHex for EC {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        //    write!(f, "{:x}", HexDisplay(self));
        todo!()
    }
}

impl Display for EC {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:X}")
    }
}

#[cfg(feature = "serde")]
impl Serialize for EC {
    fn serialize(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        unimplemented!()
    }
}
impl Add for EC {
    type Output = EC;

    fn add(self, _other: Self) -> Self {
        todo!()
    }
}

impl Mul for EC {
    type Output = EC;

    fn mul(self, _other: Self) -> Self {
        todo!()
    }
}
