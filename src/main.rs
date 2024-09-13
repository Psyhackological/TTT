mod config;
mod export_csv;
mod toc;

use anyhow::{Context, Result};
use config::TaskDefaultConfig;
use export_csv::{write_to_csv, CsvRecord};
use std::env;
use toc::extract_toc_items;

fn main() -> Result<()> {
    let current_dir = env::current_dir().with_context(|| {
        "The directory does not exist or there are insufficient permissions to access it."
    })?;

    let config_path_buf = current_dir.join("./config/defaults.toml");
    let config = TaskDefaultConfig::try_from(config_path_buf.as_path())?;

    let toc_path_buf = current_dir.join("./input/toc.txt");
    let toc_items = extract_toc_items(toc_path_buf.as_path())?;

    let records: Vec<CsvRecord> = toc_items
        .into_iter()
        .map(|toc| CsvRecord::from_config_and_toc(&config, &toc, None, None, None, None))
        .collect();

    let to_todoist_path_buf = current_dir.join("./output/to_todoist.csv");
    write_to_csv(records, to_todoist_path_buf)?;

    Ok(())
}
