use crate::errors::Errors;
use crate::operation::software_list::software_list;
use crate::operation::update_script::update_script;
use crate::{CliConfig, SystemId};
use std::process::Command;
use tokio::io::AsyncReadExt;

pub async fn update_command(software_name: &str, config: &CliConfig) -> Result<String, Errors> {
    let software = software_list(&config.url).await?;

    let soft = software
        .iter()
        .find(|this| this.long_name == software_name || this.short_name == software_name)
        .expect("Couldn't find entered software");

    let script = update_script(&SystemId(soft.uuid), config).await?;

    let output = Command::new(script).output().map_err(Errors::IoError)?;

    let mut str = String::new();

    let _ = output
        .stdout
        .as_slice()
        .read_to_string(&mut str)
        .await
        .map_err(Errors::IoError);

    Ok(str)
}
