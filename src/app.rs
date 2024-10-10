pub enum CurrentScreen {
    Main,
    Editing,
    // display a prompt asking the user whether or not they really want to inject the current token
    // data into the current repository
    Injecting,
    Deleting,
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

pub struct App {
    pub alias_input: String,
    pub username_input: String, // the currently being edited json key.
    pub email_input: String,    // the currently being edited json value.
    pub token_input: String,    // the currently being edited json value.
    // TODO: make this a hashmap such that you can re-select stuff for editing later on
    pub entries: Vec<Entry>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}

impl App {
    pub fn new() -> App {
        App {
            alias_input: String::new(),
            username_input: String::new(),
            email_input: String::new(),
            token_input: String::new(),
            entries: Vec::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
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

    // Write all entries to the corresponding files
    pub fn store_entries(&mut self) {
        // TODO: implement properly, for now just output everything to the terminal
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
        self.alias_input = String::new();
        self.username_input = String::new();
        self.email_input = String::new();
        self.token_input = String::new();
        self.currently_editing = None;
    }
}
