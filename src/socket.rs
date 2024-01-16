use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};

use crate::{auth_guard::permission_check_socket, prisma::Role};

pub struct Rooms {}

impl Rooms {
    pub const MOBILE: &'static str = "mobile";
    pub const ADMIN: &'static str = "admin";
    pub const EXEC: &'static str = "exec";
}

pub fn on_connect(socket: SocketRef, Data(_value): Data<Value>) {
    socket.on(
        "ping:mobile",
        |socket: SocketRef, Data(_value): Data<String>| async move {
            let headers = &socket.req_parts().headers;
            if !permission_check_socket(headers.clone(), Role::None).await {
                return;
            }
            socket.join(Rooms::MOBILE.to_string()).ok();
        },
    );

    socket.on(
        "ping:admin",
        |socket: SocketRef, Data(_value): Data<String>| async move {
            let headers = &socket.req_parts().headers;
            if permission_check_socket(headers.clone(), Role::Exec).await {
                socket.join(Rooms::EXEC.to_string()).ok();
            }
            if permission_check_socket(headers.clone(), Role::Team).await {
                socket.join(Rooms::ADMIN.to_string()).ok();
            }
        },
    );
}
