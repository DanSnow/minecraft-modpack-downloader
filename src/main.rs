use color_eyre::Result;
use futures::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use once_cell::sync::Lazy;
use std::{env, fs::{self, File}, sync::Arc};
use tokio::sync::Semaphore;

mod api;
mod download;
mod model;

static SEMAPHORE: Lazy<Semaphore> = Lazy::new(|| Semaphore::new(3));

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let target = ProgressDrawTarget::stdout();
    let mp = Arc::new(MultiProgress::with_draw_target(target));
    let mut file = File::open(env::args_os().nth(1).unwrap())?;
    let manifest: model::Manifest = serde_json::from_reader(&mut file)?;
    let pb = mp.add(ProgressBar::new(manifest.files.len() as u64));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );
    let mut path = env::current_dir().unwrap();
    path.push("outputs");
    fs::create_dir_all(&path)?;
    let total = manifest.files.len() as u64;
    futures::stream::iter(manifest.files.iter())
        .for_each(|&file| {
            let mp = mp.clone();
            let pb = pb.clone();
            tokio::spawn(async move {
                let _guard = SEMAPHORE.acquire().await?;
                let files = api::get_mod_files(file.project_id).await?;
                let mod_file = files
                    .into_iter()
                    .find(move |f| f.id == file.file_id)
                    .unwrap();
                let mut path = env::current_dir().unwrap();
                path.push("outputs");
                path.push(&mod_file.file_name);
                download::download(
                    mp,
                    path,
                    mod_file.download_url.clone(),
                    mod_file.file_length,
                )
                .await?;
                pb.inc(1);
                if pb.position() == total {
                    pb.finish();
                }
                Ok::<(), color_eyre::eyre::Error>(())
            });
            futures::future::ready(())
        })
        .await;
    mp.join()?;
    Ok(())
}
