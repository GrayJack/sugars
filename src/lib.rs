#![cfg_attr(not(any(feature = "nightly")), forbid(unstable_features))]

#![cfg_attr(feature = "nightly", feature(duration_float))]

mod comprehension;
mod boxed;
mod times;
