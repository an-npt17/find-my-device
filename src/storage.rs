use crate::location::Location;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn get_storage_path() -> Result<PathBuf, Box<dyn Error>> {
    let dirs = directories::ProjectDirs::from("com", "yourusername", "location_tracker")
        .ok_or("Could not determine project directories")?;

    let data_dir = dirs.data_dir();
    fs::create_dir_all(data_dir)?;

    Ok(data_dir.join("locations.jsonl"))
}

pub fn save_location(location: &Location) -> Result<(), Box<dyn Error>> {
    let path = get_storage_path()?;

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    let location_json = serde_json::to_string(location)?;
    writeln!(file, "{}", location_json)?;

    Ok(())
}
