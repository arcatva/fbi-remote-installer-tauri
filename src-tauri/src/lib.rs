use std::{os::windows::fs::MetadataExt, path::Path};
use tokio::fs;

#[derive(serde::Serialize, Debug)]
pub struct FileMeta {
    file_name: String,
    file_size: u64,
    creation_time: u64,
}

impl FileMeta {
    fn from(file_name: String, file_size: u64, creation_time: u64) -> Result<FileMeta, ()> {
        Ok(FileMeta {
            file_name,
            file_size,
            creation_time,
        })
    }
}







pub async fn list_files_with_path(path: &str) -> Result<Vec<FileMeta>, std::io::Error> {
    let path = Path::new(path);
    let mut entries = fs::read_dir(path).await?;
    let mut file_result = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let meta = entry.metadata().await?;
        file_result.push(
            FileMeta::from(
                entry.file_name().into_string().unwrap(),
                meta.file_size(),
                meta.creation_time(),
            )
            .unwrap(),
        );
    }

    Ok(file_result)
}
