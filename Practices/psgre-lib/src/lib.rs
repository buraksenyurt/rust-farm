use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

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

pub fn contact(conn: &ConnectionSettings) -> io::Result<ConnectionState> {
    let address = format!("{}:{}", conn.host, conn.port);
    let mut stream = TcpStream::connect(address)?;
    let contact_message = format!(
        "user {}\0password {}\0database {}\0",
        conn.user, conn.password, conn.database
    );
    stream.write_all(contact_message.as_bytes())?;
    let mut response = [0; 1024];
    let bytes_read = stream.read(&mut response)?;
    if bytes_read > 0 {
        return Ok(ConnectionState::Connected(format!(
            "Received: {:?}",
            &response[..bytes_read]
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
