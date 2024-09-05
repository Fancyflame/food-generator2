use std::{fs, path::Path};

use crate::syntax::SerializeMap;
use anyhow::{anyhow, Result};

pub use read::read_lib;
pub use write::save_lib;

mod read;
mod write;

pub fn save_lib_to_file<P>(lib: &SerializeMap, path: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let file = write::save_lib(lib);
    fs::write(path, file)
}

pub fn read_lib_from_file<P>(path: P) -> Result<SerializeMap>
where
    P: AsRef<Path>,
{
    let file = fs::read(path)?;
    read::read_lib(&file).ok_or_else(|| anyhow!("illegal file content"))
}

#[test]
fn test_varint() {
    let mut vec = Vec::new();
    let value = 0x4567;
    write::put_varint(&mut vec, value);

    let mut data = &vec[..];
    let read = read::get_varint(&mut data).unwrap();
    assert!(data.is_empty());
    assert_eq!(value, read);
}
