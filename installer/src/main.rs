use dirs;
use git2::Repository;
use std::env;
use std::fs::create_dir;

fn main() {
    match create_config_dir() {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    };
    create_default_files();
}

fn create_default_files() -> () {
    let gitignore_url = "https://github.com/github/gitignore#versioned-templates";
    match Repository::clone(&gitignore_url, get_gignore_path()) {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    };
}

fn create_config_dir() -> std::io::Result<()> {
    create_dir(get_gignore_path())?;
    Ok(())
}

fn get_gignore_path() -> String {
    format!("{config_path}/gignore", config_path = get_config_dir())
}

fn get_config_dir() -> String {
    let home_dir = match env::home_dir() {
        Some(path) => println!("Your home directory, probably: {}", path.display()),
        None => println!("Impossible to get your home dir!"),
    };
    dirs::config_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}
