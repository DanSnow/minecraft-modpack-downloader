use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Manifest {
    pub name: String,
    pub minecraft: Minecraft,
    pub files: Vec<Mod>,
    pub overrides: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Minecraft {
    pub version: String,
    pub mod_loaders: Vec<ModLoader>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModLoader {
    pub id: String,
    pub primary: bool,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Mod {
    #[serde(rename = "projectID")]
    pub project_id: u32,
    #[serde(rename = "fileID")]
    pub file_id: u32,
    pub required: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModFile {
    pub id: u32,
    pub display_name: String,
    pub file_name: String,
    pub file_length: u64,
    pub download_url: String,
    pub game_version: Vec<String>,
}
