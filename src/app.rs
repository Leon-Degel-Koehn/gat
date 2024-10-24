use homedir::my_home;
use regex::Regex;
use std::fs::File;
use std::process::Command;
use std::{fs, path::PathBuf};

pub enum CurrentScreen {
    Main,
    Editing,
    // display a prompt asking the user whether or not they really want to inject the current token
    // data into the current repository
    Deleting,
    Cloning,
    Injecting,
}

pub enum CurrentlyEditing {
    Alias,
    Username,
    Email,
    Token,
}

pub struct Entry {
    pub alias: String,    // an alias displayed to the user when browsing stored keys
    pub username: String, // the git username to be used for commits
    pub email: String,    // email used in commits
    pub pa_token: String, // personal access token used for login credentials in git
}

impl Entry {
    pub fn to_string(&self) -> String {
        format!(
            "{},{},{},{}",
            self.alias, self.username, self.email, self.pa_token
        )
    }
}

pub struct App {
    pub alias_input: String,
    pub username_input: String, // the currently being edited json key.
    pub email_input: String,    // the currently being edited json value.
    pub token_input: String,    // the currently being edited json value.
    pub clone_url_input: String,
    pub entries: Vec<Entry>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub selected_index: Option<usize>,
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub save_file: PathBuf,
    pub workdir: String,
}

impl App {
    pub fn new() -> App {
        let parent_dir = match my_home() {
            Ok(res) => res,
            Err(_) => None,
        };
        let mut save_file = match parent_dir {
            Some(dir) => dir,
            None => panic!("No home dir for the user exists, making profile storage impossible"),
        };
        save_file.push(".gat");
        if !save_file.exists() {
            let _ = File::create(&save_file);
        }
        let mut app = App {
            alias_input: String::new(),
            username_input: String::new(),
            email_input: String::new(),
            token_input: String::new(),
            clone_url_input: String::new(),
            entries: Vec::new(),
            current_screen: CurrentScreen::Main,
            selected_index: None,
            currently_editing: None,
            save_file: save_file.clone(),
            workdir: ".".to_string(),
        };
        app.load_entries(save_file);
        app
    }

    pub fn load_entries(&mut self, save_file: PathBuf) {
        let content = fs::read_to_string(save_file).expect("unable to read file");
        for line in content.lines() {
            if line.len() < 1 {
                continue;
            };
            let profile_split: Vec<&str> = line.split(',').collect();
            if profile_split.len() != 4 {
                continue;
            }
            self.entries.push(Entry {
                alias: String::from(profile_split[0]),
                username: String::from(profile_split[1]),
                email: String::from(profile_split[2]),
                pa_token: String::from(profile_split[3]),
            });
        }
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Alias => {
                    self.currently_editing = Some(CurrentlyEditing::Username)
                }
                CurrentlyEditing::Username => {
                    self.currently_editing = Some(CurrentlyEditing::Email)
                }
                CurrentlyEditing::Email => self.currently_editing = Some(CurrentlyEditing::Token),
                CurrentlyEditing::Token => self.currently_editing = Some(CurrentlyEditing::Alias),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Alias);
        }
    }

    pub fn store_entries(&mut self) {
        let created_entry = Entry {
            alias: self.alias_input.clone(),
            username: self.username_input.clone(),
            email: self.email_input.clone(),
            pa_token: self.token_input.clone(),
        };
        self.entries.push(created_entry);
    }

    pub fn clear(&mut self) {
        self.alias_input = String::new();
        self.username_input = String::new();
        self.email_input = String::new();
        self.token_input = String::new();
        self.clone_url_input = String::new();
        self.currently_editing = None;
    }

    pub fn str_from_entry(&self) -> String {
        match self.selected_index {
            None => String::new(),
            Some(idx) => format!(
                "Username: {}\n\nEmail: {}\n\nToken: {}",
                self.entries[idx].username, self.entries[idx].email, self.entries[idx].pa_token
            ),
        }
    }

    pub fn delete_current_entry(&mut self) {
        match self.selected_index {
            Some(idx) => {
                self.entries.remove(idx);
                if self.entries.len() <= idx {
                    self.selected_index = None;
                }
            }
            None => {}
        }
    }

    fn exec_cmd(&self, command: String) {
        let _ = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &command])
                .current_dir(self.workdir.clone())
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .args(["-c", &command])
                .current_dir(self.workdir.clone())
                .output()
                .expect("failed to execute process")
        };
    }

    pub fn inject_selected_profile(&self) {
        let Some(idx) = self.selected_index else {
            return;
        };
        let selected_entry = &self.entries[idx];
        let inject_username = format!("git config --local user.name {}", selected_entry.username);
        let inject_email = format!("git config --local user.email {}", selected_entry.email);
        self.exec_cmd(inject_username);
        self.exec_cmd(inject_email);
    }

    pub fn save_all_data(&self) {
        let mut content = String::new();
        for entry in &self.entries {
            content.push_str(format!("{}\n", entry.to_string()).as_str());
        }
        fs::write(&self.save_file, content).expect("unable to write entry to file");
    }

    pub fn clone_repo(&mut self) {
        let re = Regex::new("^https://(?<url>.+)$").unwrap();
        let Some(re_match) = re.captures(&self.clone_url_input) else {
            panic!("Illegal url input")
        };
        let url = &re_match["url"];
        let clone_command = match self.selected_index {
            Some(idx) => format!(
                "git clone https://{}:{}@{}",
                self.entries[idx].username, self.entries[idx].pa_token, url
            ),
            None => panic!("No profile selected"),
        };
        let re_path = Regex::new("(?<path>[^/]*).git").unwrap();
        let Some(path_match) = re_path.captures(url) else {
            panic!("Illegal url input")
        };
        let clone_path = &path_match["path"];
        let workdir_backup = self.workdir.clone();
        self.exec_cmd(clone_command);
        self.workdir = clone_path.to_string();
        self.inject_selected_profile();
        self.workdir = workdir_backup;
    }
}
