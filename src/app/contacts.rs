use core::panic;

use askama_axum::IntoResponse;
use axum::{extract::{Query, Path}, Form};
use diesel::prelude::*;

use crate::{models::{Contact, NewContact}, db::connection, app::templates::{get_template, contacts::{ContactsTemplate, ShowContactTemplate, DeleteContactTemplate, ContactsTable}}};

use super::templates::contacts::{NewContactTemplate, EditContactTemplate};


#[derive(Deserialize)]
pub struct Params {
    pub q: Option<String>
}

pub async fn contacts_index() -> impl IntoResponse {
    get_template(ContactsTemplate::default())
}

pub async fn contacts_search(Form(form): Form<Params>) -> impl IntoResponse {
    use crate::schema::contacts::dsl::*;
    let default_contacts: Vec<Contact> = Vec::new();

    let contacts_vector = match form.q {
        Some(query) => {
            let con = &mut connection();

                contacts
                    .filter(
                        email.like(
                            format!("%{}%", query.clone())
                        )
                    )
                    .load::<Contact>(con).unwrap()

        },
        None =>  default_contacts
    };

    get_template(ContactsTable { contacts: contacts_vector })
}

pub async fn new_contact() -> impl IntoResponse {
    get_template(NewContactTemplate::default())
}

pub async fn save_contact(Form(payload): Form<NewContact>) -> impl IntoResponse {    
    use crate::schema::contacts::dsl::*;

    let connection = &mut connection();
    let fetched_users: Result<Vec<Contact>, _> = contacts
        .filter(
            email.eq(&payload.email)
        ).load::<Contact>(connection);

    if let Ok(fetched_users) = fetched_users {
        if !fetched_users.is_empty() {
           return get_template(
            NewContactTemplate { 
                contact: Some(fetched_users.get(0).unwrap().clone()), 
                already_exists: true 
            })
        }
    }

    let new_contact = NewContact {
        first: payload.first,
        last: payload.last,
        phone: payload.phone,
        email: payload.email,
    };

    let contact_insert = diesel::insert_into(contacts)
        .values(&new_contact)
        .execute(connection);

    let fetch_last_insert = contacts.order(id.desc()).first::<Contact>(connection);
        
    if contact_insert.is_err() {
        panic!("Error inserting contact");
    }

    get_template(NewContactTemplate {
        contact: fetch_last_insert.ok().unwrap().into(),
        already_exists: false
    })
}

pub async fn show_contact(Path(user_id): Path<String>) -> impl IntoResponse {
    use crate::schema::contacts::dsl::*;   

    let con = &mut connection();
    let fetch_user = contacts
        .filter(
            id.eq(user_id.parse::<i32>().unwrap())
        ).first::<Contact>(con);

    if let Ok(user) = fetch_user { 
        return get_template(ShowContactTemplate { contact: Some(user) }) 
    } else {
        return get_template(ShowContactTemplate::default())         
    }
}

pub async fn show_edit_contact(Path(user_id): Path<String>) -> impl IntoResponse {
    use crate::schema::contacts::dsl::*;

    let con = &mut connection();
    let fetch_user_data = contacts
        .filter(
            id.eq(user_id.parse::<i32>().unwrap())
        ).first::<Contact>(con);

    if let Ok(user) = fetch_user_data {
        return get_template(EditContactTemplate { contact: Some(user) })
    } else {
        return get_template(EditContactTemplate::default())
    }
}

pub async fn edit_contact(Path(user_id): Path<String>, Form(payload): Form<NewContact>) -> impl IntoResponse {
    use crate::schema::contacts::dsl::*;
    let con = &mut connection();
    let parse_id = user_id.parse::<i32>().unwrap_or(0);

    if let Ok(user) = contacts.filter(id.eq(parse_id)).first::<Contact>(con) {
        let update_request = diesel::update(contacts.find(parse_id))
            .set((
                first.eq(payload.first),
                last.eq(payload.last),
                phone.eq(payload.phone),
                email.eq(payload.email)
            ))
            .execute(con);

        if update_request.is_err() {
            panic!("Error updating contact");
        }

        return get_template(EditContactTemplate { contact: Some(user) })
    } else {
        return get_template(EditContactTemplate::default())
    }
}

pub async fn delete_contact(Path(user_id): Path<String>) -> impl IntoResponse {
    use crate::schema::contacts::dsl::*;
    let parse_id = user_id.parse::<i32>().unwrap_or(0);

    if let Ok(_) = contacts.filter(id.eq(parse_id)).first::<Contact>(&mut connection()) {
        let delete = diesel::delete(contacts.find(parse_id)).execute(&mut connection());

        if delete.is_err() {
            panic!("Error deleting contact");
        }

        return get_template(DeleteContactTemplate { deleted: true })
    } else {
        return get_template(DeleteContactTemplate::default())
    }
}
