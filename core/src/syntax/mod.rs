use std::collections::HashMap;

#[cfg(feature = "compile")]
pub use compiler::compile;

use crate::share_str::ShareStr;

#[cfg(feature = "compile")]
mod compiler;

pub type SerializeMap = Vec<Section>;

#[derive(Debug)]
pub struct Section {
    pub encoder: Vec<Vec<Seg>>,
    pub decoder: Layer,
}

#[derive(Debug)]
pub enum Seg {
    Text(ShareStr),
    Use(u32),
}

#[derive(Debug)]
pub enum Layer {
    Branch(HashMap<char, Layer>),
    Certain(u32),
}
