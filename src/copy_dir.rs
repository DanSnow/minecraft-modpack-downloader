use std::path::Path;

use color_eyre::Result;
use tokio::fs;

#[async_recursion::async_recursion(?Send)]
pub async fn copy_dir_all(src: impl AsRef<Path> + 'static, dst: impl AsRef<Path> + 'static) -> Result<()> {
    let src = src.as_ref().to_owned();
    let dst = dst.as_ref().to_owned();
    fs::create_dir_all(&dst).await?;
    let mut entries = fs::read_dir(src).await?;
    while let Some(entry) = entries.next_entry().await? {
        let ty = entry.file_type().await?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.join(entry.file_name())).await?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name())).await?;
        }
    }
    Ok(())
}
