use tokio::net::TcpListener;

mod persistence;
mod api;
mod model;
mod filesystem;
mod initialization;

#[tokio::main]
async fn main() {
    match initialization::run_startup_sequence() {
        Ok(conn) => {
            // App is ready to run
            println!("🚀 Application started successfully.");
            // Pass `conn` to your web server or main loop here
        }
        Err(e) => {
            eprintln!("🔥 Critical startup failure: {:?}", e);
            std::process::exit(1);
        }
    }

    let app = api::app();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Server listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}