use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};

pub fn on_connect(socket: SocketRef, Data(value): Data<Value>) {
    println!("on_connect: {:?}", value);
    socket.emit("hello", "world").ok();
}
