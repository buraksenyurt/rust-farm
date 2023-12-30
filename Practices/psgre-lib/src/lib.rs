use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

const PROTOCOL_VERSION: i32 = 196608; // PostgreSql Protocol version 3.0

pub struct ConnectionSettings {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, PartialEq)]
pub enum ConnectionState {
    Connected(String),
    Failed(String),
}

fn create_handshake_message(conn: &ConnectionSettings) -> Vec<u8> {
    let mut messages = Vec::new();

    messages.extend_from_slice(&PROTOCOL_VERSION.to_be_bytes());
    messages.extend_from_slice(b"user\0");
    messages.extend_from_slice(conn.user.as_bytes());
    messages.push(0);

    messages.extend_from_slice(b"password\0");
    messages.extend_from_slice(conn.password.as_bytes());
    messages.push(0);

    messages.extend_from_slice(b"database\0");
    messages.extend_from_slice(conn.database.as_bytes());
    messages.push(0);

    messages.push(0);

    let length = (messages.len() + 4) as i32;
    let mut length_bytes = length.to_be_bytes().to_vec();
    length_bytes.append(&mut messages);

    length_bytes
}

pub fn contact(conn: &ConnectionSettings) -> io::Result<ConnectionState> {
    let address = format!("{}:{}", conn.host, conn.port);
    let mut stream = TcpStream::connect(address)?;
    let contact_message = create_handshake_message(&conn);
    stream.write_all(&contact_message)?;
    let mut response = [0; 1024];
    let bytes_read = stream.read(&mut response)?;
    if bytes_read > 0 {
        return Ok(ConnectionState::Connected(format!(
            "{}",
            String::from_utf8(response[..bytes_read].to_vec()).unwrap()
        )));
    }
    Ok(ConnectionState::Failed(
        "No response received from the Postgres server.".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contact_to_postgresql_test() {
        let conn_sets = ConnectionSettings {
            host: "localhost".to_string(),
            port: 5432,
            user: "scoth".to_string(),
            password: "tiger".to_string(),
            database: "gamersworld".to_string(),
        };
        let actual = contact(&conn_sets);
        let expected = ConnectionState::Connected("".to_string());
        assert_eq!(actual.unwrap(), expected);
    }
}
