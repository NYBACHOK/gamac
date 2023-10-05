use crate::errors::Errors;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Software {
    pub uuid: Uuid,
    pub long_name: String,
    pub short_name: String,
    pub developer: Option<String>,
    pub web_site: Option<String>,
}

pub async fn software_list(url: &Url) -> Result<Vec<Software>, Errors> {
    let response = reqwest::Client::new()
        .get(url.join("list/software").expect("Invalid url"))
        .send()
        .await
        .map_err(Errors::ReqwestError)?;

    response
        .json::<Vec<Software>>()
        .await
        .map_err(Errors::ReqwestError)
}
