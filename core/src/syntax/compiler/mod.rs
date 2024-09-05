use std::path::Path;

use anyhow::Result;

use super::SerializeMap;

mod link;
mod parse_tokens;
mod searcher;
mod serialize;

pub fn compile(base_dir: impl AsRef<Path>) -> Result<SerializeMap> {
    let expr_secs = parse_tokens::parse(base_dir)?;
    let linked_secs = link::link_secs(expr_secs)?;
    Ok(serialize::serialize(&linked_secs))
}
