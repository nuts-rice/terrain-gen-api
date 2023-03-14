use actix_web::{web, HttpResponse};

pub use core::{fmt, str};
use num_bigint::BigUint as bigint;

use std::{
    fmt::{Display, Formatter},
    ops::{Add, Mul},
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EC {
    pub p: bigint,
    pub modulus: bigint,
    pub a2: usize,
    pub a4: usize,
    pub a6: usize,
    pub is_weirstrass: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub point: Option<FinitePoint>,
    pub curve: EC,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FinitePoint {
    pub x: bigint,
    pub y: bigint,
}
#[derive(serde::Deserialize)]
pub struct FormData {
    coeffiecents: Vec<usize>,
}

impl EC {
    fn new(p: bigint, modulus: bigint, coeffiecents: &[usize]) -> Self {
        EC {
            p,
            modulus,
            a2: coeffiecents[0],
            a4: coeffiecents[1],
            a6: coeffiecents[2],
            is_weirstrass: coeffiecents[0] == 0,
        }
    }

    pub async fn reals_point_mult() -> HttpResponse {
        HttpResponse::Ok().finish()
    }
    pub async fn reals_point_add() -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    pub async fn getInputs(form: web::Form<FormData>) -> HttpResponse {
        let _coefficents = &form.coeffiecents;

        HttpResponse::Ok().finish()
    }

    pub async fn to_repr(self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "p: {}, a2: {}, a4: {}, a6: {}, modulus: {}, equation: y^2 = x^3 + 2x + 3",
            self.p, self.a2, self.a4, self.a6, self.modulus,
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

impl FinitePoint {
    fn reduce_modulo(self, modulus: &bigint) -> Self {
        Self {
            x: self.x % modulus,
            y: self.y % modulus,
        }
    }
}

impl Point {
    fn is_identity(self) -> Self {
        unimplemented!()
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
mod eea {

    fn advance_euclid() {
        unimplemented!()
    }

    fn eea() {
        unimplemented!()
    }

    pub fn mod_inv_eea() {
        unimplemented!()
    }
}
