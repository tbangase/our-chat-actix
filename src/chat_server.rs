use std::{
    collections::{HashMap, HashSet}, 
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    }
};

use actix::prelude::*;
use rand::prelude::{ThreadRng, Rng};

use crate::models::{rooms::{Room, NewRoom}, messages::Message as DBMessage};

/// Message that ChatServer sends.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// New Chat Session creating
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>
}

/// Information for Session disconnecting
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}


/// Information for Message that sends to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize,      // Id of the Clinet session
    pub msg: String,    // Peer message
    pub room: String,   // Room name
}

/// List of available room
/// Why Struct ??
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// Information for Joining to Room
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: usize,      // Client id
    pub name: String,   // Room to Join name
}


pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> ChatServer {
        // default room
        let mut rooms = HashMap::new();
        rooms.insert("World".to_string(), HashSet::new());

        match Room::get_room("World".to_string()) {
            Ok(_) => (),
            Err(_) => {
                // Create World Room if not exists.
                match Room::create(NewRoom {
                    name: "World".to_string(),
                }) {
                    Ok(val) => println!("{:?}", val),
                    Err(e) => println!("{:?}", e),
                };
            }
        }

        ChatServer {
            sessions: HashMap::new(),
            rooms,
            rng: ThreadRng::default(),
            visitor_count,
        }
    }
}

impl ChatServer {
    /// Implementation of sending message to all users in the room(chat_server)
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        // Room is name of the room 
        // and Getting room from HashMap
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(Message(message.to_string()));
                    }
                }
            }
        }
        
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

/// Handler for Connect Actor Message
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, actor_message: Connect, _: &mut Context<Self>) -> Self::Result {
        let room_name = "World".to_string();
        self.send_message(&room_name, "Someone joined", 0);

        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, actor_message.addr);
        
        self.rooms
            .entry(room_name.clone())
            .or_insert_with(HashSet::new)
            .insert(id);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_message(&room_name, &format!("Total visitors {}", count), 0);

        id
    }
}

/// Handler for Disconnect Actor Message
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, actor_message: Disconnect, _: &mut Self::Context) {
        println!("Someone disconnected.");

        let mut removed_rooms: Vec<String> = Vec::new();

        // remove address
        // 1. Delete Session from ChatServer
        if self.sessions.remove(&actor_message.id).is_some() {
            // If Client ID Exists
            // Into iterator of Rooms
            for (name, room_sessions) in &mut self.rooms {
                // 2. Delete Session from Rooms
                if room_sessions.remove(&actor_message.id) {
                    removed_rooms.push(name.to_string());
                }
            }
        }

        // send message to other users
        for room in removed_rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}

/// Handler for ClientMessage which Actor Message
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, actor_message: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&actor_message.room, actor_message.msg.as_str(), actor_message.id);
    } 
}

/// Handler for ListRooms Actor Message
impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Self::Context) -> Self::Result {
        let mut rooms = Vec::new();

        // Get Rooms from ChatServer
        for key in self.rooms.keys() {
            rooms.push(key.to_string())
        }

        MessageResult(rooms)
    }
    
}

/// Handler for Join Actor Message
impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, actor_message: Join, _: &mut Self::Context) -> Self::Result {
        let Join { id, name } = actor_message;
        let mut rooms = Vec::new();
        
        // Disconnect from Connecting rooms
        // Here will be better
        for (room_name, room_sessions) in &mut self.rooms {
            if room_sessions.remove(&id) {
                rooms.push(room_name.to_string());
            }
        }

        for room in rooms {
            self.send_message(&room, "Someone Leave this Room.", 0);
        }

        // Connect to New Room
        self.rooms
            .entry(name.clone())
            .or_insert_with(HashSet::new)
            .insert(id);

        // Send participating message to other ones.
        self.send_message(&name, "Someone come into this Room!", id);
    }
}

