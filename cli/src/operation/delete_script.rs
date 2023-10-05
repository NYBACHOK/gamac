use crate::errors::Errors;
use crate::{CliConfig, SystemId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DeleteScript {
    package_name: String,
    command: String,
    delete_instruction: String,
}

pub async fn delete_script(software_id: &SystemId, config: &CliConfig) -> Result<String, Errors> {
    let script_elements = reqwest::Client::new()
        .delete(format!(
            "{}/software/{}/system/{}",
            config.url.as_str(),
            software_id.0,
            config.system.expect("Need to set your system")
        ))
        .send()
        .await
        .map_err(Errors::ReqwestError)?
        .json::<DeleteScript>()
        .await
        .map_err(Errors::ReqwestError)?;

    let script = format!(
        "{} {} {}",
        script_elements.command, script_elements.delete_instruction, script_elements.package_name
    );

    Ok(script)
}
