use once_cell::sync::OnceCell;
use socketioxide::{SocketIo, extract::SocketRef};
use std::sync::Arc;

pub static IO_CLIENT: OnceCell<Arc<SocketIo>> = OnceCell::new();

pub async fn init_socket() -> Result<socketioxide::layer::SocketIoLayer, Box<dyn std::error::Error>>
{
    let (layer, io) = SocketIo::new_layer();

    let io_arc = Arc::new(io.clone());

    IO_CLIENT
        .set(io_arc)
        .map_err(|_| "SocketIo already initialized")?;

    io.ns("/", |s: SocketRef| async move {
        s.on("message", |s: SocketRef| async move {
            s.emit("message-back", "Hello World!").ok();
        });
    });

    Ok(layer)
}

pub async fn emit_all(event: &str, payload: impl serde::Serialize) {
    if let Some(io) = IO_CLIENT.get() {
        let _ = io.emit(event, &payload).await;
    }
}
