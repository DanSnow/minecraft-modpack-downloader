use color_eyre::{eyre::eyre, Result};
use futures_lite::{stream, StreamExt};
use gumdrop::Options;
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use once_cell::sync::{Lazy, OnceCell};
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::Semaphore;

mod api;
mod download;
mod model;

#[derive(Debug, Options)]
struct Args {
    #[options(short = "h")]
    help: bool,
    #[options(short = "d")]
    destination: Option<String>,
    #[options(free)]
    manifest_path: String,
}

#[derive(Debug)]
struct ProgressGuard {
    pb: ProgressBar,
    total: u64,
}

impl Drop for ProgressGuard {
    fn drop(&mut self) {
        self.pb.inc(1);
        if self.pb.position() == self.total {
            self.pb.finish();
        }
    }
}

static SEMAPHORE: Lazy<Semaphore> = Lazy::new(|| Semaphore::new(3));

#[tokio::main]
async fn main() -> Result<()> {
    static OUTPUT: OnceCell<PathBuf> = OnceCell::new();
    color_eyre::install()?;
    let args = Args::parse_args_default_or_exit();
    let target = ProgressDrawTarget::stdout();
    let mp = Arc::new(MultiProgress::with_draw_target(target));
    let mut file = File::open(&args.manifest_path)?;
    let manifest: model::Manifest = serde_json::from_reader(&mut file)?;
    let pb = mp.add(ProgressBar::new(manifest.files.len() as u64));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );
    let path = create_output_path(&args, &manifest.name)?;
    fs::create_dir_all(&path)?;
    let _ = OUTPUT.set(path);
    let total = manifest.files.len() as u64;
    stream::iter(manifest.files.iter())
        .for_each(|&file| {
            let mp = mp.clone();
            let pb_guard = ProgressGuard {
                pb: pb.clone(),
                total,
            };
            tokio::spawn(async move {
                let _guard = SEMAPHORE.acquire().await?;
                let files = api::get_mod_files(file.project_id).await?;
                let mod_file = files
                    .into_iter()
                    .find(move |f| f.id == file.file_id)
                    .ok_or_else(|| eyre!("can not found specific file for {}", file.project_id))?;
                let mut path = OUTPUT.get().unwrap().clone();
                path.push(&mod_file.file_name);
                let model::ModFile {
                    download_url,
                    file_length,
                    ..
                } = mod_file;
                download::download(mp, path, download_url, file_length).await?;
                // explicit move progress bar guard
                drop(pb_guard);
                Ok::<(), color_eyre::eyre::Error>(())
            });
        })
        .await;
    mp.join()?;
    Ok(())
}

fn create_output_path(args: &Args, name: &str) -> Result<PathBuf> {
    let cwd = env::current_dir()?;
    let mut path = args.destination.as_ref().map(PathBuf::from).unwrap_or(cwd);
    path.push(name);
    path.push("mods");
    Ok(path)
}
