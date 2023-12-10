use std::default;

use askama::Template;

use crate::models::Contact;

#[derive(Template)]
#[template(path="contacts/index.html")]
pub struct ContactsTemplate {
    pub q: String
}

impl default::Default for ContactsTemplate {
    fn default() -> Self {
        ContactsTemplate { q: String::from("") }
    }
}

#[derive(Template)]
#[template(path="contacts/table.html")]
pub struct ContactsTable {
    pub contacts: Vec<Contact>,
}

impl default::Default for ContactsTable{
    fn default() -> Self {
        ContactsTable { contacts: Vec::new() }
    }
}

#[derive(Template)]
#[template(path="contacts/new.html")]
pub struct NewContactTemplate {
    pub contact: Option<Contact>,
    pub already_exists: bool
}

impl default::Default for NewContactTemplate {
    fn default() -> Self {
        NewContactTemplate { contact: None, already_exists: false }
    }
}

#[derive(Template)]
#[template(path="contacts/show.html")]
pub struct ShowContactTemplate {
    pub contact: Option<Contact>
}

impl default::Default for ShowContactTemplate {
    fn default() -> Self {
        ShowContactTemplate { contact: None }
    }
}

#[derive(Template)]
#[template(path="contacts/edit.html")]
pub struct EditContactTemplate {
    pub contact: Option<Contact>
}

impl default::Default for EditContactTemplate {
    fn default() -> Self {
        EditContactTemplate { contact: None }
    }
}

#[derive(Template)]
#[template(path="contacts/delete.html")]
pub struct DeleteContactTemplate {
    pub deleted: bool
}

impl default::Default for DeleteContactTemplate {
    fn default() -> Self {
        DeleteContactTemplate { deleted: false }
    }
}
