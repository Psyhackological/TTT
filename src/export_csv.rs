use std::fs::File;
use std::path::Path;

use crate::config::TaskDefaultConfig;
use crate::toc::{TaskType, TocItem};
use anyhow::{Context, Result};
use csv::Writer;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CsvRecord {
    #[serde(rename = "TYPE")]
    task_type: TaskType,

    #[serde(rename = "CONTENT")]
    content: String,

    #[serde(rename = "DESCRIPTION")]
    description: Option<String>,

    #[serde(rename = "PRIORITY")]
    priority: Option<u8>,

    #[serde(rename = "INDENT")]
    indent: Option<u8>,

    #[serde(rename = "AUTHOR")]
    author: Option<String>,

    #[serde(rename = "RESPONSIBLE")]
    responsible: Option<String>,

    #[serde(rename = "DATE")]
    date: Option<String>,

    #[serde(rename = "DATE_LANG")]
    date_lang: Option<String>,

    #[serde(rename = "TIMEZONE")]
    timezone: Option<String>,

    #[serde(rename = "DURATION")]
    duration: Option<u16>,

    #[serde(rename = "DURATION_UNIT")]
    duration_unit: Option<String>,
}

impl CsvRecord {
    pub fn from_config_and_toc(
        config: &TaskDefaultConfig,
        toc_item: &TocItem,
        description: Option<String>,
        responsible: Option<String>,
        date: Option<String>,
        date_lang: Option<String>,
    ) -> Self {
        match toc_item.task_type {
            TaskType::Section => Self {
                task_type: toc_item.task_type.clone(),
                content: toc_item.content.clone(),
                description: None,
                priority: None,
                indent: None,
                author: None,
                responsible: None,
                date: None,
                date_lang: None,
                timezone: None,
                duration: None,
                duration_unit: None,
            },
            TaskType::Task => Self {
                task_type: toc_item.task_type.clone(),
                content: toc_item.content.clone(),
                description,
                priority: Some(config.priority),
                indent: toc_item.indent,
                author: Some(config.author.clone()),
                responsible,
                date,
                date_lang,
                timezone: Some(config.timezone.clone()),
                duration: Some(config.duration),
                duration_unit: Some(config.duration_unit.clone()),
            },
        }
    }
}

pub fn write_to_csv<P: AsRef<Path>>(records: Vec<CsvRecord>, path: P) -> Result<()> {
    let file = File::create(path).context("Failed to create a file.")?;
    let mut writer = Writer::from_writer(file);

    for record in records {
        writer
            .serialize(record)
            .context("Failed to write record {record}")?;
    }

    writer.flush()?;
    Ok(())
}
