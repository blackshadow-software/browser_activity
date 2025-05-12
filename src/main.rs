use std::io::Error as IoError;
use std::net::TcpListener;
use super_minimal_ws_server::{set_status, Status, WsResponse};
use tungstenite::{accept, error::Error as WsError, Message};

fn main() {
    if let Err(e) = run_super_minimal_ws_server("8235") {
        eprintln!("❌ Server error: {}", e);
    }
}

pub fn run_super_minimal_ws_server(port: &str) -> Result<(), IoError> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr)?;
    println!("🚀 Listening on ws://{}", addr);

    for tcp_stream in listener.incoming() {
        let tcp_stream = match tcp_stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("⚠️  Failed TCP accept: {}", e);
                continue;
            }
        };
        let peer = tcp_stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "<unknown>".into());
        set_status(&Status::Active);
        println!("🔌 TCP connection {:?}", Status::Active);

        let mut ws = match accept(tcp_stream) {
            Ok(ws) => {
                println!("✅ WebSocket handshake completed: {}", peer);
                ws
            }
            Err(e) => {
                eprintln!("⚠️  Handshake failed for {}: {}", peer, e);
                continue;
            }
        };

        loop {
            match ws.read() {
                Ok(Message::Text(text)) => match serde_json::from_str::<WsResponse>(&text) {
                    Ok(client_msg) => {
                        let status: Status = client_msg.status;
                        set_status(&status);
                        println!("➡️ Browser sent status = {:?}", status);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse client JSON `{}`: {}", text, e);
                    }
                },
                Err(WsError::ConnectionClosed) | Err(WsError::Protocol(_)) | Err(WsError::Utf8) => {
                    set_status(&Status::Inactive);
                    println!("❌ Connection: {:?}", Status::Inactive);
                    break;
                }
                Err(e) => {
                    eprintln!("‼ Error on {}: {}", peer, e);
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
