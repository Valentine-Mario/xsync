use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, prelude::*, BufRead};
use std::{fs, io::Error, path::Path};
use toml_edit::{value, Document, Table};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub folders: HashMap<String, String>,
    pub files: HashMap<String, String>,
}

pub enum FolderConfig {
    Add(String),
    Remove(String),
}

const checksum_file: &str = ".xsync.toml";
const ignore_file: &str = ".xsync";

pub fn create_checksum_file(path: &Path) -> Result<(), Error> {
    let folder_path = format!("{}/{}", path.to_str().unwrap(), checksum_file);
    if !Path::new(&folder_path).exists() {
        let mut file = fs::File::create(folder_path)?;
        let config = Config {
            folders: HashMap::new(),
            files: HashMap::new(),
        };
        let toml = toml::to_string(&config).unwrap();
        file.write_all(toml.as_bytes())?;
    }
    Ok(())
}

pub fn get_ignore_file(path: &Path) -> Result<Vec<String>, Error> {
    let folder_path = format!("{}/{}", path.to_str().unwrap(), ignore_file);
    let mut all_lines = vec![];
    if Path::new(&folder_path).exists() {
        if let Ok(lines) = read_lines(folder_path) {
            for line in lines {
                all_lines.push(line?)
            }
            return Ok(all_lines);
        }
    }
    Ok(vec![])
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_checksum_file(path: &Path) -> Result<String, Error> {
    let folder_path = format!("{}/{}", path.to_str().unwrap(), checksum_file);
    let data = fs::read_to_string("file_name.txt")?;
    Ok(data)
}

pub fn parse_checksum_config(data: String) -> Result<Config, String> {
    let raw_cfg: Result<Config, toml::de::Error> = toml::from_str(&data);
    let raw_cfg = match raw_cfg {
        Ok(raw_cfg) => raw_cfg,
        Err(_) => return Err(String::from("Error parsing config")),
    };
    Ok(raw_cfg)
}

pub fn update_folder_config(
    data: String,
    key: &str,
    path: &Path,
    action: &FolderConfig,
) -> Result<(), Error> {
    let mut doc = data.parse::<Document>().expect("invalid toml document");

    let folder_path = format!("{}/{}", path.to_str().unwrap(), checksum_file);

    match action {
        FolderConfig::Add(item) => {
            doc[key][item] = value(item.to_string());
            doc[key].as_inline_table_mut().map(|t| t.fmt());
            fs::write(&folder_path, doc.to_string())?;
            Ok(())
        }
        FolderConfig::Remove(item) => {
            let mut a = parse_checksum_config(data).unwrap();
            if key == "folders" {
                a.folders.remove(item);
                let toml_str = toml::to_string(&a).unwrap();
                doc[key].as_inline_table_mut().map(|t| t.fmt());
                fs::write(&toml_str, doc.to_string())?;
            } else {
                a.files.remove(item);
                let toml_str = toml::to_string(&a).unwrap();
                doc[key].as_inline_table_mut().map(|t| t.fmt());
                fs::write(&toml_str, doc.to_string())?;
            }

            Ok(())
        }
    }
}