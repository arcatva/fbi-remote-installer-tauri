use chrono::DateTime;
use std::{fmt::Debug, os::windows::fs::MetadataExt, path::Path};
use tokio::fs;


#[derive(serde::Serialize, Debug)]
pub struct FileMeta {
    file_name: String,
    file_size: u64,
    modified: String,
}

impl FileMeta {
    fn from(
        file_name: String,
        file_size: u64,
        modified: String,
    ) -> Result<FileMeta, std::io::Error> {
        Ok(FileMeta {
            file_name,
            file_size,
            modified,
        })
    }
}

pub async fn list_files(path: &str) -> Result<Vec<FileMeta>, std::io::Error> {
    let path = Path::new(path);
    let mut entries = fs::read_dir(path).await?;
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let meta = entry.metadata().await?;
        files.push(FileMeta::from(
            entry.file_name().into_string().unwrap(),
            meta.file_size(),
            {
                let time: DateTime<chrono::Local> = meta.modified().unwrap().into();
                time.format("%Y-%m-%d").to_string()
            },
        )?);
    }
    Ok(files)
}
