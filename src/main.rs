use dirs;
use std::env;
use std::fs::copy;
use std::process;

fn main() {
    if args().len() == 1 {
        println!("Please write valid gitignore name");
        process::exit(1);
    }
    let filename = &args()[1];
    let _path: String = get_gignore_path();
    println!("{}", _path);
    match copy_gitignore(filename) {
        Ok(_) => (),
        Err(_) => println!("This gitignore does not exists"),
    };
}

fn args() -> Vec<String> {
    env::args().collect()
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

fn copy_gitignore(gitignore_name: &String) -> std::io::Result<()> {
    copy(
        format!(
            "{config_dir}/{gitignore_name}.gitignore",
            gitignore_name = gitignore_name,
            config_dir = get_gignore_path()
        ),
        format!(".gitignore"),
    )?;
    Ok(())
}
