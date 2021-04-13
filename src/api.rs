use crate::model::ModFile;
use color_eyre::Result;

pub async fn get_mod_files(id: u32) -> Result<Vec<ModFile>> {
    let val = reqwest::get(format!(
        "https://addons-ecs.forgesvc.net/api/v2/addon/{}/files",
        id
    ))
    .await?
    .json::<Vec<ModFile>>()
    .await?;
    Ok(val)
}
