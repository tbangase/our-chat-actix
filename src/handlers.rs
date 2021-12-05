use std::time::Instant;
use actix::Addr;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;

use crate::{chat_server, ws_chat_session::WsChatSession};

pub async fn handshake_chat(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<chat_server::ChatServer>>
) -> Result<HttpResponse, Error>{
    println!("WebSocket Connecting...");
    let result = ws::start(
        WsChatSession{
            id: 0,
            hb: Instant::now(),
            room: "Main".to_string(),
            name: None,
            addr: server.get_ref().clone(),
        }, 
        &req, 
        stream
    );
    println!("{:?}", result);
    result
}
