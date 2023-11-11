use std::fs;

#[derive(PartialEq, Debug)]
pub enum FsEntityStatus {
    IsFile,
    IsDir,
    NotFound,
}

pub fn get_fs_entity_status(path: &str) -> FsEntityStatus {
    match fs::metadata(format!("./{}", path)) {
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

#[cfg(test)]
mod tests {
    use crate::file::*;

    #[test]
    fn mime() {
        assert_eq!(get_mine_type(String::from("test.txt")), "text/plain");
        assert_eq!(get_mine_type(String::from("test.html")), "text/html");
        assert_eq!(
            get_mine_type(String::from("test")),
            "application/octet-stream"
        );
    }

    #[test]
    fn file_status() {
        assert_eq!(get_fs_entity_status("Cargo.toml"), FsEntityStatus::IsFile);
        assert_eq!(get_fs_entity_status("src"), FsEntityStatus::IsDir);
        assert_eq!(get_fs_entity_status("IDONTEXIST"), FsEntityStatus::NotFound);
        assert_eq!(get_fs_entity_status("/Cargo.toml"), FsEntityStatus::IsFile);
        assert_eq!(get_fs_entity_status("/src"), FsEntityStatus::IsDir);
        assert_eq!(
            get_fs_entity_status("/IDONTEXIST"),
            FsEntityStatus::NotFound
        );
    }
}
