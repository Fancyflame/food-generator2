use std::path::Path;

use anyhow::Result;

pub use serialize::{Layer, Section, Seg};

mod link;
mod parse_tokens;
mod searcher;
mod serialize;

pub type SerializeMap = Vec<Section>;

pub fn parse(base_dir: impl AsRef<Path>) -> Result<SerializeMap> {
    let expr_secs = parse_tokens::parse(base_dir)?;
    let linked_secs = link::link_secs(expr_secs)?;
    Ok(serialize::serialize(&linked_secs))
}
