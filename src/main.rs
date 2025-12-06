use tokio::net::TcpListener;

mod persistence;
mod api;
mod model;
mod filesystem;

#[tokio::main]
async fn main() {
    let app = api::app();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Server listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}