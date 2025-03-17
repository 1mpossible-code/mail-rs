use imap::Session;
use native_tls::TlsStream;
use std::net::TcpStream;

pub fn get_messages(sequence_set: &str, imap_session: &mut Session<TlsStream<TcpStream>>) -> imap::error::Result<Vec<String>> {
    imap_session.select("Inbox")?;

    let messages = imap_session.fetch(sequence_set, "RFC822")?;
    let mut bodies: Vec<String> = vec![];
    for msg in messages.iter() {
        let body = msg.body().expect("Failed to get message body");
        bodies.push(String::from_utf8(body.to_vec()).expect("Failed to convert message body to string"));
    }

    Ok(bodies)
}

pub fn login(
    imap_server: &str,
    imap_port: u16,
    username: &str,
    password: &str
) -> Result<Session<TlsStream<TcpStream>>, Box<dyn std::error::Error>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    let client = imap::connect((imap_server, imap_port), imap_server, &tls).unwrap();

    let imap_session = client
        .login(username, password)
        .map_err(|e| e.0).expect("Failed to create imap session");

    Ok(imap_session)
}