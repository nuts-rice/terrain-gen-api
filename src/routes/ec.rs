use actix_web::{web, HttpResponse};
use anyhow::{anyhow, Result};

pub use core::{fmt, str};
use num_bigint::BigUint as bigint;

use std::{
    fmt::{Display, Formatter},
    ops::{Add, Deref, Mul},
};

use crate::{THREE, TWO};

#[derive(Clone, Debug, Default)]
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

    pub async fn contains_inner(&self, point: &FinitePoint) -> bool {
        let p = &self.modulus;
        let lhs = point.y.modpow(&TWO, p);
        //checks for elliptic equation
        let rhs = point.x.modpow(&THREE, p)
            + self.a2 * point.x.modpow(&TWO, p)
            + self.a4 * &point.x
            + self.a6;
        lhs == rhs
    }
}

impl PartialEq for EC {
    fn eq(&self, other: &Self) -> bool {
        self.a2 == other.a2
            && self.a4 == other.a4
            && self.a6 == other.a6
            && self.modulus == other.modulus
    }
}

impl Point {
    pub async fn new(curve: EC, coords: Option<FinitePoint>) -> Result<Self> {
        match coords {
            Some(point) => {
                if curve.contains_inner(&point).await {
                    let point = Point {
                        point: Some(point.reduce_modulo(&curve.modulus)),
                        curve,
                    };
                    Ok(point)
                } else {
                    Err(anyhow!("point not on curve"))
                }
            }
            None => Ok(Point { point: None, curve }),
        }
    }
}

impl Deref for Point {
    type Target = FinitePoint;

    fn deref(&self) -> &Self::Target {
        self.point.as_ref().unwrap()
    }
}

impl FinitePoint {
    pub fn reduce_modulo(self, modulus: &bigint) -> Self {
        Self {
            x: self.x % modulus,
            y: self.y % modulus,
        }
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
    use super::*;

    fn advance_euclid(a: &mut bigint, old_a: &mut bigint, quoatiant: bigint) {
        let temp = a.clone();
        *a = &*old_a + quoatiant * &temp;
        *old_a = temp;
    }

    fn eea(a: bigint, b: bigint) -> (bigint, bigint, bigint) {
        let (mut old_r, mut rem) = (a, b);
        let (mut old_s, mut coeff_s) = (bigint::from(1u32), bigint::from(0u32));
        let (mut old_t, mut coeff_t) = (bigint::from(0u32), bigint::from(1u32));

        while rem != bigint::from(0u32) {
            let quotiant = old_r.clone() / rem.clone();
            advance_euclid(&mut rem, &mut old_r, quotiant.clone());
            advance_euclid(&mut coeff_s, &mut old_s, quotiant.clone());
            advance_euclid(&mut coeff_t, &mut old_t, quotiant);
        }

        (old_r, old_s, old_t)
    }

    pub fn mod_inv_eea(x: bigint, n: bigint) -> Option<bigint> {
        let (g, x, _) = eea(x, n.clone());
        if g == bigint::from(1u32) {
            Some((x % n.clone() + n.clone()) % n)
        } else {
            None
        }
    }
}
