use crate::errors::Errors;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Systems {
    pub uuid: Uuid,
    pub os_name: String,
    web_site: Option<String>,
}

pub async fn systems_list(url: &Url) -> Result<Vec<Systems>, Errors> {
    let response = reqwest::Client::new()
        .get(url.join("list/system").expect("Invalid url"))
        .send()
        .await
        .map_err(Errors::ReqwestError)?;

    response
        .json::<Vec<Systems>>()
        .await
        .map_err(Errors::ReqwestError)
}
