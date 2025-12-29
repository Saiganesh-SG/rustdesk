use crate::client::*;
use async_trait::async_trait;
use hbb_common::{
    config::PeerConfig,
    config::READ_TIMEOUT,
    futures::{SinkExt, StreamExt},
    log,
    message_proto::*,
    protobuf::Message as _,
    rendezvous_proto::ConnType,
    tokio::{self, sync::mpsc},
    Stream,
};
use std::sync::{Arc, RwLock};

/// CLI Session for command-line interface interactions
#[derive(Clone)]
pub struct CliSession {
    id: String,
    lc: Arc<RwLock<LoginConfigHandler>>,
    sender: mpsc::UnboundedSender<Data>,
    password: String,
}

impl CliSession {
    pub fn new(id: &str, password: Option<String>, sender: mpsc::UnboundedSender<Data>) -> Self {
        let pwd = password.unwrap_or_else(|| {
            // Try to load saved password first
            let config = PeerConfig::load(id);
            if config.password.is_empty() {
                rpassword::prompt_password("Enter password: ").unwrap_or_default()
            } else {
                String::new()
            }
        });

        let session = Self {
            id: id.to_owned(),
            sender,
            password: pwd,
            lc: Default::default(),
        };
        
        session.lc.write().unwrap().initialize(
            id.to_owned(),
            ConnType::DEFAULT_CONN,
            None,
            false,
            None,
            None,
            None,
        );
        session
    }
}

#[async_trait]
impl Interface for CliSession {
    fn get_lch(&self) -> Arc<RwLock<LoginConfigHandler>> {
        self.lc.clone()
    }

    fn msgbox(&self, msgtype: &str, title: &str, text: &str, link: &str) {
        match msgtype {
            "input-password" => {
                self.sender
                    .send(Data::Login((String::new(), String::new(), self.password.clone(), true)))
                    .ok();
            }
            "re-input-password" => {
                log::error!("{}: {}", title, text);
                match rpassword::prompt_password("Enter password: ") {
                    Ok(password) => {
                        let login_data = Data::Login((String::new(), String::new(), password, true));
                        self.sender.send(login_data).ok();
                    }
                    Err(e) => {
                        log::error!("Re-input password failed: {:?}", e);
                    }
                }
            }
            msg if msg.contains("error") => {
                log::error!("[{}] {}: {}", msgtype, title, text);
            }
            _ => {
                log::info!("[{}] {}: {}", msgtype, title, text);
            }
        }
    }

    fn handle_login_error(&self, err: &str) -> bool {
        handle_login_error(self.lc.clone(), err, self)
    }

    fn handle_peer_info(&self, pi: PeerInfo) {
        self.lc.write().unwrap().handle_peer_info(&pi);
        log::info!("Connected to peer: {}", pi.username);
    }

    fn set_multiple_windows_session(&self, _sessions: Vec<WindowsSession>) {
        // Not needed for CLI
    }

    async fn handle_hash(&self, pass: &str, hash: Hash, peer: &mut Stream) {
        handle_hash(self.lc.clone(), pass, hash, self, peer).await;
    }

    async fn handle_login_from_ui(
        &self,
        os_username: String,
        os_password: String,
        password: String,
        remember: bool,
        peer: &mut Stream,
    ) {
        handle_login_from_ui(
            self.lc.clone(),
            os_username,
            os_password,
            password,
            remember,
            peer,
        )
        .await;
    }

    async fn handle_test_delay(&self, t: TestDelay, peer: &mut Stream) {
        handle_test_delay(t, peer).await;
    }

    fn send(&self, data: Data) {
        self.sender.send(data).ok();
    }
}

/// Connect to a remote system
#[tokio::main(flavor = "current_thread")]
pub async fn connect(
    remote_id: &str,
    id_server: Option<String>,
    relay_server: Option<String>,
    key: String,
    password: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Configure custom servers if provided
    if let Some(server) = id_server {
        hbb_common::config::Config::set_option("custom-rendezvous-server".to_owned(), server.clone());
    }
    
    if let Some(relay) = relay_server {
        hbb_common::config::Config::set_option("relay-server".to_owned(), relay.clone());
    }

    let (sender, mut receiver) = mpsc::unbounded_channel::<Data>();
    let handler = CliSession::new(remote_id, password, sender);
    
    let token = hbb_common::config::LocalConfig::get_option("access_token");

    log::info!("Connecting to remote system: {}", remote_id);
    
    match crate::client::Client::start(remote_id, &key, &token, ConnType::DEFAULT_CONN, handler.clone()).await {
        Err(err) => {
            log::error!("Failed to connect to {}: {}", remote_id, err);
            return Err(err.into());
        }
        Ok(((mut stream, direct), _)) => {
            log::info!("Connected successfully! (direct: {})", direct);
            
            // Main event loop for handling messages
            loop {
                tokio::select! {
                    res = hbb_common::timeout(READ_TIMEOUT, stream.next()) => match res {
                        Err(_) => {
                            log::error!("Connection timeout");
                            break;
                        }
                        Ok(Some(Ok(bytes))) => {
                            if let Ok(msg_in) = Message::parse_from_bytes(&bytes) {
                                handle_message(msg_in, &handler, &mut stream).await;
                            }
                        }
                        Ok(None) => {
                            log::info!("Connection closed by remote");
                            break;
                        }
                        _ => {}
                    },
                    data = receiver.recv() => {
                        match data {
                            Some(Data::Close) => {
                                log::info!("Closing connection");
                                break;
                            }
                            Some(Data::Login((os_username, os_password, password, remember))) => {
                                // Handle login
                                let mut msg_out = Message::new();
                                let mut lr = LoginRequest {
                                    username: os_username.clone(),
                                    password: password.into_bytes(),
                                    ..Default::default()
                                };
                                msg_out.set_login_request(lr);
                                let bytes = msg_out.write_to_bytes()?;
                                stream.send(&bytes).await?;
                            }
                            _ => {}
                        }
                    }
                }
            }
            
            Ok(())
        }
    }
}

async fn handle_message(
    msg: Message,
    handler: &CliSession,
    stream: &mut Stream,
) {
    match msg.union {
        Some(message::Union::Hash(hash)) => {
            log::info!("Received hash from remote");
            let password = handler.password.clone();
            handler.handle_hash(&password, hash, stream).await;
        }
        Some(message::Union::LoginResponse(lr)) => {
            if lr.error.is_empty() {
                log::info!("Login successful!");
            } else {
                log::error!("Login error: {}", lr.error);
            }
        }
        Some(message::Union::PeerInfo(pi)) => {
            handler.handle_peer_info(pi);
        }
        Some(message::Union::Misc(misc)) => {
            log::info!("Received misc message");
        }
        _ => {
            log::debug!("Received other message type");
        }
    }
}

/// Execute a command on the remote system
#[tokio::main(flavor = "current_thread")]
pub async fn execute_command(
    command: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // This will be implemented to execute commands on an already-connected session
    // For now, return a placeholder
    log::info!("Executing command: {}", command);
    Ok(format!("Command '{}' execution not yet implemented", command))
}
