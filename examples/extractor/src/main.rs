use eserde::Deserialize;
use eserde_axum::Json;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .route("/user", axum::routing::post(create_user));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

#[derive(Deserialize)]
pub struct Contact {
    pub email: String,
    pub phone: String,
}

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub contact: Contact,
}

// This handler demonstrates successful deserialization of a JSON payload into `User`.
//
// Example valid request:
//   curl -X POST http://127.0.0.1:3000/user \
//        -H "Content-Type: application/json" \
//        -d '{
//              "name": "Alice",
//              "age": 30,
//              "contact": { "email": "alice@example.com", "phone": "123-456-7890" }
//            }'
//
// When invalid JSON or a payload that cannot be deserialized into `User` is sent,
// the extractor responds with a ProblemDetails error that includes an HTTP status
// code. For example, sending invalid JSON might produce a response like:
//
//   HTTP/1.1 400 Bad Request
//   Content-Type: application/problem+json
// 
//   {
//     "type": "about:blank",
//     "title": "Invalid request body",
//     "status": 400,
//     "detail": "Failed to deserialize JSON request body into `User`."
//   }
//
// This illustrates how error responses include proper status codes via ProblemDetails.
async fn create_user(Json(payload): Json<User>) -> axum::http::StatusCode {
    println!("User: {}", payload.name);
    axum::http::StatusCode::CREATED
}
