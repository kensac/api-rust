use std::fmt::{Display, Formatter};

use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    layer::SocketIoLayer,
    SocketIo,
};

use crate::auth_guard::permission_check_socket;

pub enum Rooms {
    Mobile,
    Admin,
    Exec,
}

impl Rooms {
    fn from_string(room: &str) -> Option<Rooms> {
        match room {
            "mobile" => Some(Rooms::Mobile),
            "admin" => Some(Rooms::Admin),
            "exec" => Some(Rooms::Exec),
            _ => None,
        }
    }
}

impl Display for Rooms {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rooms::Mobile => write!(f, "mobile"),
            Rooms::Admin => write!(f, "admin"),
            Rooms::Exec => write!(f, "exec"),
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

    socket.on(
        "ping:mobile",
        |socket: SocketRef, Data(_value): Data<String>| {
            socket.join(Rooms::Mobile.to_string()).ok();
        },
    );

    socket.on(
        "ping:admin",
        |socket: SocketRef, Data(_value): Data<String>| async move {
            let headers = &socket.req_parts().headers;
            if !permission_check_socket(headers.clone(), vec!["Exec".to_string()]).await {
                socket.emit("error", "Unauthorized").ok();
                return;
            }
            socket.join(Rooms::Admin.to_string()).ok();
            socket.emit("ping:admin", "Pong").ok();
        },
    );
}

pub fn get_socket_layer() -> SocketIoLayer {
    let (socket_layer, io) = SocketIo::new_layer();

    io.ns("/socket", on_connect);

    socket_layer
}
