use axum::Json;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Health {
    pub status: i32,
    pub message: String
}

pub async fn health() -> Json<Health> {
    Json(Health {
        status: 200,
        message: "OK".to_string()
    })
}