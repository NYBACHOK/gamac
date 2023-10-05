use clap::Parser;
use cli::command::delete::delete_command;
use cli::command::install::install_command;
use cli::command::select_system::select_system_command;
use cli::command::software_list::software_command;
use cli::command::system_list::systems_command;
use cli::command::update::update_command;
use cli::errors::Errors;
use cli::{Args, CliConfig, Operation};
use config::{Config, FileFormat};

const CONFIG_NAME: &str = ".capstone_config.toml";

fn main() -> Result<(), Errors> {
    let home_dir = match dirs::home_dir() {
        None => Err(Errors::CustomError(
            "Failed to get home directory".to_string(),
        )),
        Some(path) => Ok(path),
    }?;

    let args = Args::parse();

    let config = {
        let mut config = Config::builder().add_source(config::File::new(
            home_dir.join(CONFIG_NAME).to_str().expect("Invalid path"),
            FileFormat::Toml,
        ));

        if let Some(path) = args.config_file.clone() {
            config = config.add_source(config::File::new(
                path.to_str().expect("Invalid path"),
                FileFormat::Toml,
            ));
        }

        let settings = config.build().map_err(Errors::ConfigError)?;

        settings
            .try_deserialize::<CliConfig>()
            .map_err(Errors::ConfigError)?
    };

    let rt = tokio::runtime::Runtime::new().expect("Failed to start runtime");

    let result = match args.operation {
        Operation::Install { name } => rt.block_on(install_command(&name, &config))?,
        Operation::Delete { name } => rt.block_on(delete_command(&name, &config))?,
        Operation::Update { name } => rt.block_on(update_command(&name, &config))?,
        Operation::Software => rt.block_on(software_command(&config))?,
        Operation::Systems => rt.block_on(systems_command(&config))?,
        Operation::Select { name } => rt.block_on(select_system_command(
            &name,
            &config,
            args.config_file.unwrap_or(home_dir.join(CONFIG_NAME)),
        ))?,
    };

    println!("{result}");

    Ok(())
}
