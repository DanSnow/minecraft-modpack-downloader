use color_eyre::Result;
use reqwest::StatusCode;
use tap::prelude::*;
use tracing::error;

use crate::model::ModFile;

pub async fn get_mod_files(id: u32) -> Result<Vec<ModFile>> {
    let url = format!("https://addons-ecs.forgesvc.net/api/v2/addon/{}/files", id);
    let val = reqwest::get(url)
        .await?
        .tap(|response| match response.status() {
            StatusCode::OK => (),
            code => {
                error!("unexpected response code {} when fetching mod id {}", code, id);
            }
        })
        .json::<Vec<ModFile>>()
        .await?;
    Ok(val)
}
