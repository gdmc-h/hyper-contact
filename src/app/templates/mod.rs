use askama::Template;
use askama_axum::IntoResponse;
use axum::{http::StatusCode, response::Html};

pub mod contacts;

pub fn get_template<T: Template>(template: T) -> impl IntoResponse {
    let html = match template.render() {
        Ok(h) => h,
        Err(e) => panic!("{}", e),
    };

    (StatusCode::OK, Html(html).into_response())
    
}