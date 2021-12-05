use actix_web_actors::ws;
use actix::*;
use std::time::{Instant, Duration};
use crate::chat_server::{ChatServer, self};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsChatSession {
    pub id: usize,      // Session ID (Unique)
    pub hb: Instant,    // Heart Beat
    pub room: String,   // Joined Room name
    pub name: Option<String>,       // Peer name ?
    pub addr: Addr<ChatServer>,     // Address of ChatServer (Actor)
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.hb(context);

        let addr = context.address();
        self.addr
            .send(chat_server::Connect { 
                addr: addr.recipient() 
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(context);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        self.addr.do_send(chat_server::Disconnect {
            id: self.id
        });
        Running::Stop
    }
}

impl Handler<chat_server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: chat_server::Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(
        &mut self, 
        actor_message: Result<ws::Message, ws::ProtocolError>, 
        context: &mut Self::Context
    ) {
        let actor_message = match actor_message {
            Ok(msg) => msg,
            Err(e) => {
                println!("{:?}", e);
                context.stop();
                return;
            }
        };

        println!("{:?}", actor_message);

        match actor_message {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                context.pong(&msg)
            },
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            },
            ws::Message::Binary(bytes) => println!("{:?}", bytes),
            ws::Message::Continuation(_) => {
                context.stop()
            },
            ws::Message::Close(reason) => {
                context.close(reason);
                context.stop();
            },
            ws::Message::Text(text) => {
                let message = text.trim();
                let message = if let Some(ref name) = self.name {
                    format!("{}: {}", name, message)
                } else {
                    message.to_string()
                };

                self.addr.do_send(chat_server::ClientMessage {
                    id: self.id,
                    msg: message,
                    room: self.room.clone(),
                })
            },
            ws::Message::Nop => (),
        }
        
    }
}

impl WsChatSession {
    fn hb(&self, context: &mut ws::WebsocketContext<Self>) {
        context.run_interval(HEARTBEAT_INTERVAL, |act, context| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {

                println!("Websocket Client heartbeat faild, disconnecting.");
                act.addr.do_send(chat_server::Disconnect { id: act.id });

                context.stop();

                return;
            }

            context.ping(b"");
        });
    }
}


