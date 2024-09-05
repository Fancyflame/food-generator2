use std::{io::Write, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use food_generator2::{
    decode_mode, encode_mode,
    file::{read_lib_from_file, save_lib_to_file},
    syntax::compile,
};

#[derive(clap::Parser)]
pub enum Cli {
    Compile {
        source_dir: PathBuf,
        save_file: PathBuf,
    },
    Encode {
        lib: PathBuf,
        text: String,
    },
    Decode {
        lib: PathBuf,
        text: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let (lib_path, text) = match &cli {
        Cli::Compile {
            source_dir,
            save_file,
        } => {
            print!("正在编译...");
            flush()?;
            let result = save_lib_to_file(&compile(source_dir)?, save_file);
            match result {
                Ok(_) => println!("完成"),
                Err(_) => {
                    println!("失败");
                }
            }
            result?;
            return Ok(());
        }
        Cli::Encode { lib, text } | Cli::Decode { lib, text } => (lib, text),
    };

    let lib = if lib_path.is_dir() {
        compile(lib_path)?
    } else {
        read_lib_from_file(lib_path)?
    };

    let output = match cli {
        Cli::Encode { .. } => encode_mode(&lib, text)?,
        Cli::Decode { .. } => decode_mode(&lib, text)?,
        _ => unreachable!(),
    };

    println!("{output}");

    Ok(())
}

fn flush() -> std::io::Result<()> {
    std::io::stdout().flush()
}
