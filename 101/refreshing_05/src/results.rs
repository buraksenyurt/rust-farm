#[test]
fn send_message_http_test() -> Result<(), SendResponse> {
    send_message("http://google.com", "{'id':1}")?;
    Ok(())
}
#[test]
fn send_message_https_test() -> Result<(), SendResponse> {
    send_message("https://google.com", "{'id':1}")?;
    Ok(())
}

#[test]
fn send_message_with_empty_body_test() -> Result<(), SendResponse> {
    send_message("https://google.com", "")?;
    Ok(())
}
fn send_message(address: &str, body: &str) -> Result<(), SendResponse> {
    dbg!("Sending..");
    if !address.starts_with("https://") {
        return Err(SendResponse::ProtocolError);
    }
    if body.is_empty() {
        return Err(SendResponse::InvalidBody);
    }
    Ok(())
}

#[derive(Debug)]
enum SendResponse {
    Ok,
    NotConnected,
    NotFound,
    ProtocolError,
    InvalidBody,
}
