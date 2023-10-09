use crate::errors::Errors;
use crate::operation::software_list::software_list;
use crate::CliConfig;

pub async fn software_command(config: &CliConfig) -> Result<String, Errors> {
    let software = software_list(&config.url).await?;

    let mut table = format!(
        "{:<50} | {:<50} | {:<20} \n",
        "Id", "Long name", "Short name"
    );

    software.into_iter().for_each(|this| {
        table.push_str(&format!(
            "{:<50} | {:<50} | {:<20}\n",
            this.uuid, this.long_name, this.short_name
        ))
    });

    Ok(table)
}
