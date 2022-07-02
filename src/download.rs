use std::{cmp, path::Path, sync::Arc};

use color_eyre::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use tokio::{fs::File, io::AsyncWriteExt};

static STYLE: Lazy<ProgressStyle> = Lazy::new(|| {
    ProgressStyle::default_bar()
        .template("{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-")
});

pub async fn download(mp: Arc<MultiProgress>, path: impl AsRef<Path> + 'static, url: String, len: u64) -> Result<()> {
    let path = path.as_ref();
    let mut res = reqwest::get(&url).await?;
    let len = res.content_length().unwrap_or(len);
    let mut file = File::create(&path).await?;
    let pb = mp.add(ProgressBar::new(len));
    pb.set_style(STYLE.clone());
    let name = path.file_name().unwrap().to_str().unwrap().to_owned();
    pb.set_message(name.clone());
    let mut downloaded = 0;
    while let Some(chunk) = res.chunk().await? {
        downloaded += chunk.len();
        file.write_all(&chunk).await?;
        pb.set_position(cmp::min(downloaded as u64, len));
    }
    pb.println(format!("finish download {}", name));
    pb.finish_and_clear();
    Ok(())
}
