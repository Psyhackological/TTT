use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

/// Enum for Task Type
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum TaskType {
    Section,
    Task,
}

/// Struct to represent each record in the CSV
#[derive(Debug, Serialize, Deserialize)]
struct TaskRecord {
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
    author: String,

    #[serde(rename = "RESPONSIBLE")]
    responsible: Option<String>,

    #[serde(rename = "DATE")]
    date: Option<DateTime<Utc>>,

    #[serde(rename = "DATE_LANG")]
    date_lang: Option<String>,

    #[serde(rename = "TIMEZONE")]
    timezone: Option<String>,

    #[serde(rename = "DURATION")]
    duration: Option<u64>,

    #[serde(rename = "DURATION_UNIT")]
    duration_unit: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::create("output.csv")?;
    let mut wtr = csv::Writer::from_writer(file);

    // Example data creation
    let tasks = vec![
        TaskRecord {
            task_type: TaskType::Section,
            content: String::from("Part 1 Introducting Rust"),
            description: None,
            priority: None,
            indent: None,
            author: String::new(),
            responsible: None,
            date: None,
            date_lang: None,
            timezone: None,
            duration: None,
            duration_unit: None,
        },
        TaskRecord {
            task_type: TaskType::Task,
            content: String::from("3.1 Using plain functions to experiment with an API"),
            description: None,
            priority: Some(4),
            indent: Some(1),
            author: String::from("Konrad (27918041)"),
            responsible: None,
            date: None,
            date_lang: None,
            timezone: Some(String::from("Europe/Warsaw")),
            duration: None,
            duration_unit: None,
        },
        // Add more tasks here as needed
    ];

    // Serialize tasks to CSV
    for task in tasks {
        wtr.serialize(task)?;
    }

    wtr.flush()?;
    Ok(())
}
