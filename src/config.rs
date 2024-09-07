use std::{error::Error, fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

fn default_ext() -> String {
    String::from("md")
}

fn default_output_path() -> String {
    String::from(".")
}

fn default_required() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontmatterField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(default)]
    pub placeholder: String,
    #[serde(default = "default_required")]
    pub required: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_ext")]
    pub ext: String,
    #[serde(default = "default_output_path", rename = "outputPath")]
    pub output_path: String,
    #[serde(default)]
    pub frontmatter: Vec<FrontmatterField>
}

pub fn read_from_json() -> Result<Config, Box<dyn Error>> {
    let file = File::open("quickmd.config.json")?;
    let reader = BufReader::new(file);
    let parsed = serde_json::from_reader(reader)?;
    Ok(parsed)
}
