use crate::errors::Errors;
use crate::operation::select_system::select_system;
use crate::operation::systems_list::systems_list;
use crate::CliConfig;
use std::path::Path;

pub async fn select_system_command(
    name: &str,
    config: &CliConfig,
    config_path: impl AsRef<Path>,
) -> Result<String, Errors> {
    let systems = systems_list(&config.url)
        .await?
        .iter()
        .find(|this| this.os_name == name)
        .cloned()
        .expect("Failed to find this system");

    select_system(&systems.uuid, config_path)?;

    Ok("Selected".to_string())
}
