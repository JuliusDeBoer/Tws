use std::fs;

#[derive(PartialEq)]
pub enum FsEntityStatus {
    IsFile,
    IsDir,
    NotFound
}

#[derive(Clone)]
pub struct File {
    pub id: String,
    pub content: Option<Vec<u8>>
}

pub fn get_metadata(path: &str) -> Result<File, anyhow::Error> {
    Ok(File {
        id: path.to_string(),
        content: None
    })
}

pub fn get_fs_entity_status(path: &str) -> FsEntityStatus {
    let metadata = fs::metadata(path);
    match metadata {
        Err(..) => FsEntityStatus::NotFound,
        Ok(v) => {
            if v.is_file() {
                FsEntityStatus::IsFile
            } else if v.is_dir() {
                FsEntityStatus::IsDir
            } else {
                FsEntityStatus::NotFound
            }
        }
    }
}

pub fn get_mine_type(path: String) -> String {
    let mime_type_guess = mime_guess::from_path(path);
    mime_type_guess.first_or_octet_stream().to_string()
}
