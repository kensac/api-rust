use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    layer::SocketIoLayer,
    SocketIo,
};

use crate::{auth_guard::permission_check_socket, base_types::AppState, prisma::Role};

pub enum Rooms {
    Mobile,
    Admin,
    Exec,
}

impl Rooms {
    fn to_string(&self) -> String {
        match self {
            Rooms::Mobile => "mobile".to_string(),
            Rooms::Admin => "admin".to_string(),
            Rooms::Exec => "exec".to_string(),
        }
    }

    fn from_string(room: &str) -> Option<Rooms> {
        match room {
            "mobile" => Some(Rooms::Mobile),
            "admin" => Some(Rooms::Admin),
            "exec" => Some(Rooms::Exec),
            _ => None,
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
            }
            socket.join(Rooms::Admin.to_string()).ok();
            socket.emit("ping:admin", "Pong").ok();
        },
    );
}

pub fn get_socket_layer(app_state: AppState) -> SocketIoLayer {
    let (socket_layer, io) = SocketIo::new_layer();

    io.ns("/socket", on_connect);

    socket_layer
}