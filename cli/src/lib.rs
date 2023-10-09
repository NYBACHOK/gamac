use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;
use uuid::Uuid;

pub mod command;
pub mod errors;
pub mod operation;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    pub config_file: Option<PathBuf>,
    #[command(subcommand)]
    pub operation: Operation,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Operation {
    Install { name: String },
    Delete { name: String },
    Update { name: String },
    Software,
    Systems,
    Select { name: String },
}

#[derive(Serialize, Deserialize)]
pub struct CliConfig {
    pub system: Option<Uuid>,
    pub url: Url,
}

pub struct SystemId(pub Uuid);
pub struct SoftwareId(pub Uuid);
