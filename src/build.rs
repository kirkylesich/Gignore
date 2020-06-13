use dirs;
use git2::Repository;
use std::fs::create_dir;

fn main() {
    create_config_dir();
    create_default_files();
}

fn create_default_files() -> () {
    let gitignore_url = "https://github.com/github/gitignore#versioned-templates";
    Repository::clone(&gitignore_url, get_gignore_path());
}

fn create_config_dir() -> std::io::Result<()> {
    create_dir(get_gignore_path())?;
    Ok(())
}

fn get_gignore_path() -> String {
    format!("{config_path}/gignore", config_path = get_config_dir())
}

fn get_config_dir() -> String {
    dirs::config_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}
