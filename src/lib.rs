// #![feature(slice_pattern)]

// #![feature(lazy_cell)]
// use std::sync::LazyLock;
// use num::BigUint as bigint; 



pub mod configuration;
pub mod domain;
pub mod routes;
pub mod startup;
pub mod telemetry;

// pub (crate) static ZERO: LazyLock<bigint> = LazyLock::new(|| bigint::from(0u32));
// pub (crate) static ONE: LazyLock<bigint> = LazyLock::new(|| bigint::from(1u32));
// pub (crate) static TWO: LazyLock<bigint> = LazyLock::new(|| bigint::from(2u32));
// pub (crate) static THREE: LazyLock<bigint> = LazyLock::new(|| bigint::from(3u32));

