use directories::UserDirs;

pub fn get_desktop_path() -> String {
    if let Some(user_dir) = UserDirs::new() {
        if let Some(path) = user_dir.desktop_dir() {
            let name = String::from(r"\new.epub");

            String::from(path.to_str().unwrap()) + name.as_str()
        } else {
            panic!("Could not find the desktop directory.");
        }
    } else {
        panic!("Could not determine user directories.");
    }
}
