use std::fs;

pub struct FileUtils;

impl FileUtils {
    pub fn file_exists(file_path: &str, create_if_not_exist: bool) -> Result<bool, std::io::Error> {
        let path = std::path::Path::new(file_path);

        if path.exists() {
            Ok(true)
        } else {
            if create_if_not_exist {
                match fs::File::create(path) {
                    Ok(_) => Ok(false),
                    Err(err) => Err(err),
                }
            } else {
                Ok(false)
            }
        }
    }

    pub fn directory_exists(
        dir_path: &str,
        create_if_not_exist: bool
    ) -> Result<bool, std::io::Error> {
        let path = std::path::Path::new(dir_path);

        if path.exists() {
            Ok(true)
        } else {
            if create_if_not_exist {
                match fs::create_dir(path) {
                    Ok(_) => Ok(false),
                    Err(err) => Err(err),
                }
            } else {
                Ok(false)
            }
        }
    }
}
