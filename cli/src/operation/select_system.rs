use crate::errors::Errors;
use crate::CliConfig;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::path::Path;
use uuid::Uuid;

pub fn select_system(id: &Uuid, path: impl AsRef<Path>) -> Result<(), Errors> {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .map_err(Errors::IoError)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(Errors::IoError)?;

    let mut data = toml::from_str::<CliConfig>(&contents).map_err(Errors::TomlErrorDe)?;

    data.system = Some(id.clone());

    file.set_len(0).map_err(Errors::IoError)?;
    file.rewind().map_err(Errors::IoError)?;

    file.write_all(
        toml::to_string(&data)
            .map_err(Errors::TomlErrorSer)?
            .as_bytes(),
    )
    .map_err(Errors::IoError)?;

    Ok(())
}
