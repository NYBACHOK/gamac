use crate::errors::Errors;
use crate::{CliConfig, SystemId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct InstallScript {
    package_name: String,
    command: String,
    install_instruction: String,
}

pub async fn install_script(software_id: &SystemId, config: &CliConfig) -> Result<String, Errors> {
    let script_elements = reqwest::Client::new()
        .put(format!(
            "{}/software/{}/system/{}",
            config.url.as_str(),
            software_id.0,
            config.system.expect("Need to set your system")
        ))
        .send()
        .await
        .map_err(Errors::ReqwestError)?
        .json::<InstallScript>()
        .await
        .map_err(Errors::ReqwestError)?;

    let script = format!(
        "{} {} {}",
        script_elements.command, script_elements.install_instruction, script_elements.package_name
    );

    Ok(script)
}
