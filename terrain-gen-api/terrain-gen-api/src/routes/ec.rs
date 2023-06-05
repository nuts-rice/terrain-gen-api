use actix_web::{web, HttpResponse};
use anyhow::{anyhow, Ok, Result};

pub use core::{fmt, str};
use num::BigUint as bigint;

use std::{
    fmt::{Display, Formatter},
    ops::{Add, Deref, Mul, Neg},
};

use yew::prelude::*;

use crate::{ONE, THREE, TWO, ZERO};

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

pub enum Msg {
    Add,
    Mul,
    Reset,
    KeyDown(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FinitePoint {
    pub x: bigint,
    pub y: bigint,
}

//need to convert from int to hex
#[derive(serde::Deserialize)]
pub struct FormData {
    coeffiecents: Vec<usize>,
}

pub async fn init_curve(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
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

    pub async fn getInputs(self, form: web::Form<FormData>) -> HttpResponse {
        let _coefficents = &form.coeffiecents;
        /* match &self.a2{
            Some(result) => {
                _a2 = result.replace
            }
        }
        */
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

    fn update() {
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

    pub async fn is_infinite(&self) -> bool {
        self.point.is_none()
    }

    pub async fn to_inner(&self) -> Option<&FinitePoint> {
        self.point.as_ref()
    }
}

impl<'a> Neg for &'a Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

//#[async_trait]
impl Add for Point {
    type Output = Result<Point>;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
        /*        if self.curve != rhs.curve {
            panic!("can't add points on different curves")
        }

        //0 + Q = Q
        //identity rule
        if self.is_infinite().await {
            return Ok(rhs).unwrap();
        } else if rhs.is_infinite().await {
            return Ok(self).unwrap()
        // P - P = 0
        } else if self == -&rhs.clone() {
            return Ok(Point::new(self.curve, None).await.unwrap());
        }
        //P + P =
        let p = &self.curve.modulus;
        let ll: &bigint = if self == rhs {
            if self.y == *ZERO {
                return Ok(Point::new(self.curve, None).await.unwrap());
            }
            let l = TWO.clone() * self.y.clone();
            let l_inv: bigint = mod_inv_eea(l, p.clone()).await;
            &((&THREE.clone() * &self.x.pow(2) + TWO.clone() * self.curve.a2 * &self.x + self.curve.a4) * l_inv)
        } else {
            let l_inv = mod_inv_eea(&rhs.x - &self.x, p.clone()).await;
            &((&rhs.y - &self.y) * l_inv)
        };

        let x = ll.pow(2) - self.curve.a2 - &self.x - &rhs.x;
        let y = ll * (&self.x - &x)   - &self.y;
        Point::new(self.curve.clone(), Some(FinitePoint {x: x % p, y: y % p})).await
        */
    }
}

impl Deref for Point {
    type Target = FinitePoint;

    fn deref(&self) -> &Self::Target {
        self.point.as_ref().unwrap()
    }
}

impl Component for Point {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            point: Some(FinitePoint {
                x: bigint::from(0u32),
                y: bigint::from(0u32),
            }),
            curve: EC::new(bigint::from(0u32), bigint::from(0u32), &[0]),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        todo!();
        //        html! {
        //            <div>
        //                <p>{self.point}</p>
        //            </div>
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
pub mod eea {

    use super::*;

    async fn advance_euclid(a: &mut bigint, old_a: &mut bigint, quoatiant: bigint) {
        dbg!(&a, &old_a, &quoatiant);
        let temp = a.clone();
        *a = &*old_a + quoatiant * &temp;
        *old_a = temp;
    }

    /*
        pub async fn inverse(a: bigint) {
            const M: bigint = bigint::from(1u32);
            let (mut b, mut x, mut y) = (M.clone(), ONE.clone(), ZERO.clone());
            while (a != ONE.clone()) {
                const quoatiant: bigint = b/a;
                y -= quoatiant * x;
                b %= a;
                std::mem::swap(&a, &b);
                std::mem::swap(&x, &y);
            }
            return (x % M + M) % M;

        }
    */

    pub async fn eea(a: bigint, b: bigint) -> (bigint, bigint, bigint) {
        let (mut old_r, mut rem) = if a > b { (b, a) } else { (a, b) };
        assert!(rem > old_r && old_r > *ZERO);
        let (mut old_s, mut coeff_s) = (ONE.clone(), ZERO.clone());
        let (mut old_t, mut coeff_t) = (ZERO.clone(), ONE.clone());

        while rem != *ONE {
            let quotiant = old_r.clone() / rem.clone();

            advance_euclid(&mut rem, &mut old_r, quotiant.clone()).await;
            advance_euclid(&mut coeff_s, &mut old_s, quotiant.clone()).await;
            advance_euclid(&mut coeff_t, &mut old_t, quotiant).await;
        }

        (old_r, old_s, old_t)
    }

    pub async fn mod_inv_eea(x: bigint, n: bigint) -> bigint {
        let (_g, x, _) = eea(x, n.clone()).await;
        let canidate = (x % n.clone() + n.clone()) % n.clone();
        if canidate != *ONE {
            dbg!(canidate.clone());
            &n - canidate
        } else {
            canidate
        }
    }
}

//small embedded tests for eea utils
/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::eea::mod_inv_eea;
        //eea::mod_inv_eea,


    #[actix_rt::test]
    async fn mod_inv_test() {
        let right_a = bigint::from(397u32);
        let right_b = bigint::from(2357u32);
        let left = bigint::from(1603u32);
        assert_eq!(left, mod_inv_eea(right_a, right_b).await);
    }



    #[actix_rt::test]
    async fn eea_test() {
        let right_a = bigint::from(68u32);
        let right_b = bigint::from(20u32);
        let right_eea = eea(right_a, right_b).await;
        assert_eq!(bigint::from(4u32), right_eea.0)
    }
}
*/
