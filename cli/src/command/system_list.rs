use crate::errors::Errors;
use crate::operation::systems_list::systems_list;
use crate::CliConfig;

pub async fn systems_command(config: &CliConfig) -> Result<String, Errors> {
    let systems = systems_list(&config.url).await?;

    let mut table = format!(
        "{:<50} | {:<50} | {:<20} \n",
        "Id", "Long name", "Short name"
    );

    systems
        .into_iter()
        .for_each(|this| table.push_str(&format!("{:<50} | {:<50} \n", this.uuid, this.os_name)));

    Ok(table)
}
