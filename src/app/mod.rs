mod health;
mod contacts;
mod templates;

use axum::{Router, routing::{get, post}, response::Redirect};
use axum_test::TestServer;

use self::{health::health, contacts::{contacts_index, new_contact, save_contact, show_contact, show_edit_contact, edit_contact, delete_contact, contacts_search}};

pub async fn start() {
    let app = routes();

    let listener = match tokio::net::TcpListener::bind("127.0.0.1:8080").await {
        Ok(server) => {
            println!("listening on {}", server.local_addr().unwrap());

            server
        },
        Err(e) => panic!("server could not be started:\n{}", e)
    };

    match axum::serve(listener, app).await {
        Ok(_) => println!("axum is serving"),
        Err(e) => panic!("axum dropped the ball:\n{}", e)
   };

}

fn routes() -> Router {
    Router::new()
    .route("/api/health", get(health))
    .route("/", get(|| async { Redirect::permanent("/contacts")}))
    .route("/contacts", get(contacts_index))
    .route("/contacts", post(contacts_search))
    .route("/contacts/:user_id/view", get(show_contact))
    .route("/contacts/new", get(new_contact))
    .route("/contacts/new", post(save_contact))
    .route("/contacts/:user_id/edit", get(show_edit_contact))
    .route("/contacts/:user_id/edit", post(edit_contact))
    .route("/contacts/:user_id/delete", post(delete_contact))
}



#[tokio::test]
async fn it_should_be_healthy() {
    let app = routes();
    let server = TestServer::new(app).unwrap();

    let res = server.get("/api/health").await;
    let expected_response = health::Health {
        status: 200,
        message: "OK".to_string()
    };
    let json = res.json::<health::Health>();

    assert_eq!(json, expected_response)
}
