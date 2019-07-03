// #![cfg_attr(not(any(feature = "nightly")), forbid(unstable_features))]

#![cfg_attr(feature = "nightly", feature(duration_float))]

mod boxed;
mod hash;
mod collections;
mod comprehension;
mod times;
