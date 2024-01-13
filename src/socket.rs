use std::fmt::{Display, Formatter};

use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use std::fmt;

use crate::{auth_guard::permission_check_socket, prisma::Role};

pub enum Rooms {
    Mobile,
    Admin,
    Exec,
}

impl Rooms {
    fn _from_string(room: &str) -> Option<Self> {
        match room {
            "mobile" => Some(Self::Mobile),
            "admin" => Some(Self::Admin),
            "exec" => Some(Self::Exec),
            _ => None,
        }
    }
}

impl Display for Rooms {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mobile => write!(formatter, "mobile"),
            Self::Admin => write!(formatter, "admin"),
            Self::Exec => write!(formatter, "exec"),
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
