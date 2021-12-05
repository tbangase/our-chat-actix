use std::sync::{Arc, atomic::AtomicUsize};

use actix_web::{App, HttpServer, HttpResponse, web};
use our_chat::{
    chat_server::ChatServer, 
    handlers::handshake_chat,
    config::Config,
};
use actix::Actor;

use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::from_filename("/.envs/.local/.rust").ok();
    let config = Config::from_env().unwrap();

    // App state
    // For keeping a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // Start chat server actor
    // Here, We need Chat Server actor.
    let server = ChatServer::new(app_state.clone()).start();

    println!("Server Running.");
    
    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .data(server.clone())
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "/static/websocket.html")
                    .finish()
            })))
            .service(web::resource("/ws/").to(handshake_chat))
            .service(fs::Files::new("/static/", "static/"))
    })
    .bind(config.server_addr.clone())?
    .run()
    .await
}
