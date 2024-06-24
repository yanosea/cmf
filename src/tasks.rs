use std::fs::File;
use std::io::Read;
use toml::Value;

pub fn get_tasks_from_makefile() -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open("Makefile.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let value: Value = toml::from_str(&contents).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to parse TOML : {}", err),
        )
    })?;
    let tasks = value["tasks"].as_table().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to read tasks from TOML")
    })?;

    Ok(tasks.keys().map(|s| s.to_string()).collect())
}
