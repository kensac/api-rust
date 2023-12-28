use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};


pub enum Rooms {
    Mobile,
    Admin,
    Exec
}

impl Rooms {
    fn to_string(&self) -> String {
        match self {
            Rooms::Mobile => "mobile".to_string(),
            Rooms::Admin => "admin".to_string(),
            Rooms::Exec => "exec".to_string()
        }
    }

    fn from_string(room: &str) -> Option<Rooms> {
        match room {
            "mobile" => Some(Rooms::Mobile),
            "admin" => Some(Rooms::Admin),
            "exec" => Some(Rooms::Exec),
            _ => None
        }
    }
}

pub fn on_connect(socket: SocketRef, Data(_value): Data<Value>) {
    socket.on("message", |socket: SocketRef, Data(value): Data<String>| {
        socket.emit("message", value).ok();
    });

    socket.on("join", |socket: SocketRef, Data(value): Data<String>| {
        if let Some(room) = Rooms::from_string(&value) {
            socket.join(room.to_string()).ok();
        }
    });

    socket.on("leave", |socket: SocketRef, Data(value): Data<String>| {
        if let Some(room) = Rooms::from_string(&value) {
            socket.leave(room.to_string()).ok();
        }
    });

    socket.on("ping:mobile", |socket: SocketRef, Data(_value): Data<String>| {
        socket.join(Rooms::Mobile.to_string()).ok();
    });

    socket.on("ping:admin", |socket: SocketRef, Data(_value): Data<String>| {
        socket.join(Rooms::Admin.to_string()).ok();
    });
}
