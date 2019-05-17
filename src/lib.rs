#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
#[macro_use]
extern crate stdweb;

pub mod ga;
pub mod genes;
pub mod params;
pub mod phenotype;
pub mod species;
pub mod utils;
