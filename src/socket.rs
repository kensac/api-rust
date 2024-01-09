use std::fmt::{Display, Formatter};

use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};

use crate::{auth_guard::permission_check_socket, prisma::Role};

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
    socket.on(
        "ping:mobile",
        |socket: SocketRef, Data(_value): Data<String>| async move {
            let headers = &socket.req_parts().headers;
            if !permission_check_socket(headers.clone(), Role::None).await {
                return;
            }
            socket.join(Rooms::Mobile.to_string()).ok();
        },
    );

    socket.on(
        "ping:admin",
        |socket: SocketRef, Data(_value): Data<String>| async move {
            let headers = &socket.req_parts().headers;
            if permission_check_socket(headers.clone(), Role::Exec).await {
                socket.join(Rooms::Exec.to_string()).ok();
            }
            if permission_check_socket(headers.clone(), Role::Team).await {
                socket.join(Rooms::Admin.to_string()).ok();
            }
        },
    );
}
