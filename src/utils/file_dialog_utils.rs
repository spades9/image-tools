use rfd::FileDialog;

pub fn open_folder() -> String {
    let files = FileDialog::new()
    .set_directory("/")
    .pick_folder();
    if let Some(path) = files {
        return path.as_os_str().to_str().unwrap().to_string();
    }
    "".to_string()
    
}



