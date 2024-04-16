use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::daemons::server::core::ipc::{parse_request, Command};

use super::ipc::UptimeCommand;

pub struct ClientRequest {
    pub client_version: String,
    pub server_version: String,
    pub command: String,
}

pub async fn accept_connections(listener: tokio::net::TcpListener, uptime_command: UptimeCommand) {
    loop {
        if let Ok((mut socket, _)) = listener.accept().await {
            let uptime_command_clone = uptime_command.clone(); // Clone the uptime_command
            tokio::spawn(async move {
                if let Err(e) = handle_client(&mut socket, &uptime_command_clone).await {
                    eprintln!("Failed to handle client: {}", e);
                }
            });
        } else {
            eprintln!("Failed to accept connection");
        }
    }
}

pub async fn handle_client(
    socket: &mut tokio::net::TcpStream,
    uptime_command: &UptimeCommand,
) -> io::Result<()> {
    println!("Client {:?} connected", socket.peer_addr()?);
    let mut buffer = [0; 1024];
    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                // Connection closed by client
                println!("Client {:?} disconnected", socket.peer_addr()?);
                return Ok(());
            }
            Ok(bytes_read) => {
                // Parse client request
                let request_str = match std::str::from_utf8(&buffer[..bytes_read]) {
                    Ok(s) => s,
                    Err(_) => {
                        eprintln!("Invalid UTF-8 received");
                        continue;
                    }
                };

                if let Some(request) = parse_request(request_str, "1.0") {
                    // Enforce version requirement
                    if request.client_version != request.server_version {
                        if let Err(e) = socket
                            .write_all(b"Client and server versions mismatch\n")
                            .await
                        {
                            eprintln!("Failed to write data to socket: {}", e);
                            return Err(e);
                        }
                        continue;
                    }

                    // Execute command
                    let response = Command::from_str(&request.command).execute(&uptime_command);

                    // Send response back to the client
                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        eprintln!("Failed to write data to socket: {}", e);
                        return Err(e);
                    }
                } else {
                    // Invalid request
                    if let Err(e) = socket.write_all(b"Invalid request\n").await {
                        eprintln!("Failed to write data to socket: {}", e);
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                return Err(e);
            }
        }
    }
}
