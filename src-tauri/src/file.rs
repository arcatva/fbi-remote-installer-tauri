use std::{fmt::Debug, os::windows::fs::MetadataExt, path::Path};

use chrono::DateTime;
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

pub async fn list_files(path: &str) -> Result<Vec<FileMeta>, String> {
	let path = Path::new(path);
	let mut entries = fs::read_dir(path).await.map_err(|e| { e.to_string() })?;
	let mut files = Vec::new();


	while let Some(entry) =
		entries
			.next_entry()
			.await.map_err(|e| { e.to_string() })?
	{
		if is_pkg(&entry.file_name().into_string().unwrap()) {
			let meta = entry.metadata().await.map_err(|e| { e.to_string() })?;
			files.push(FileMeta::from(
				entry.file_name().into_string().unwrap(),
				meta.file_size(),
				{
					let time: DateTime<chrono::Local> = meta.modified().unwrap().into();
					time.format("%Y-%m-%d").to_string()
				},
			).map_err(|e| { e.to_string() })?);
		}
	}
	Ok(files)
}

fn is_pkg(file_name: &str) -> bool {
	let extends = ["cia", "3dsx", "cetk", "tik"];
	for i in extends {
		if file_name.ends_with(i) == true {
			println!("{}", file_name);
			return true;
		}
	}
	false
}