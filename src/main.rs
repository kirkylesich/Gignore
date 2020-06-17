use git2::Repository;
use std::env::args;
use std::env::var;
use std::fs::remove_dir_all;
use std::process;

fn main() {
    let app = App {};
    let args: Vec<String> = args().collect();
    let gitignore_command = &args[1];
    match gitignore_command.as_ref() {
        "--update" => app.update_files(),
        _ => match app.copy_to_dir(gitignore_command) {
            Ok(_) => (),
            Err(e) => println!("Error: {}\nTry to 'gignore --update'", e),
        },
    }
}

struct App {}

impl App {
    fn get_config_path(&self) -> String {
        format!("/home/{username}/.config", username = self.get_username())
    }

    fn get_username(&self) -> String {
        match var("LOGNAME") {
            Ok(u) => u,
            Err(_) => "username".to_string(),
        }
    }

    fn get_path_gitignore(&self, gitignore_name: &String) -> String {
        format!(
            "{config_path}/gignore/{gitignore_name}.gitignore",
            config_path = self.get_config_path(),
            gitignore_name = gitignore_name
        )
    }

    fn copy_to_dir(&self, filename: &String) -> std::io::Result<()> {
        std::fs::copy(self.get_path_gitignore(filename), ".gitignore")?;
        Ok(())
    }

    fn remove_gitignore_files(&self) -> std::io::Result<()> {
        remove_dir_all(format!(
            "{config_path}/gignore",
            config_path = self.get_config_path()
        ))?;
        Ok(())
    }

    fn update_files(&self) -> () {
        match self.remove_gitignore_files() {
            Ok(_) => (),
            Err(_) => (),
        }
        let url = "https://github.com/github/gitignore.git";
        match Repository::clone(
            url,
            format!(
                "{config_path}/gignore",
                config_path = self.get_config_path()
            ),
        ) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
    }
}
