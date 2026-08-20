use anyhow::{Result, bail};
use std::{env, fs};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    layout: Vec<LayoutItem>,
}

#[derive(Debug, Deserialize)]
struct LayoutItem {
    name: String,
    width: u32,
    fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
struct Field {
    name: String,
    bits: u32,
}

fn main() -> Result<()> {
    let file_path = match env::args().nth(1) {
        None => bail!("please provide a file"),
        Some(path) => path,
    };

    let content = fs::read_to_string(file_path)?;
    let config = toml::from_str::<Config>(&content)?;

    for layout in config.layout {
        let mut consumed = 0;

        println!("{}", layout.name);
        println!("----------------------------------");

        for field in layout.fields {
            consumed += field.bits;
            let shift = layout.width - consumed;
            let mask = (((1u32 << field.bits) - 1) as u16) << shift;

            let hex_width = (layout.width / 4 + 2) as usize;
            let bin_width = layout.width as usize;
            println!(
                "{:<6} | {:#0hex_width$x} | {:0bin_width$b}",
                field.name, mask, mask
            );
        }
    }

    Ok(())
}
