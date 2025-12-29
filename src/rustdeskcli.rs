use clap::{Arg, Command};
use hbb_common::{config::LocalConfig, env_logger::*, log};
use librustdesk::cli_controller;

fn main() {
    // Initialize logging
    init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "info"));

    // Initialize global state
    if !librustdesk::common::global_init() {
        eprintln!("Global initialization failed.");
        return;
    }

    let app = Command::new("rustdeskcli")
        .version(librustdesk::VERSION)
        .author("Purslane Ltd <info@rustdesk.com>")
        .about("RustDesk CLI - Command-line interface for remote desktop control")
        .arg(
            Arg::new("remoteId")
                .long("remoteId")
                .value_name("REMOTE_ID")
                .help("Remote system ID to connect to")
                .required_unless_present("command"),
        )
        .arg(
            Arg::new("idServer")
                .long("idServer")
                .value_name("ID_SERVER")
                .help("ID/Rendezvous server address (e.g., 35.222.12.44)")
                .requires("remoteId"),
        )
        .arg(
            Arg::new("relayServer")
                .long("relayServer")
                .value_name("RELAY_SERVER")
                .help("Relay server address (e.g., 35.222.12.44)")
                .requires("remoteId"),
        )
        .arg(
            Arg::new("key")
                .long("key")
                .value_name("KEY")
                .help("Server public key for authentication")
                .default_value(""),
        )
        .arg(
            Arg::new("password")
                .long("password")
                .value_name("PASSWORD")
                .help("Password for authentication (optional, will prompt if not provided)")
                .requires("remoteId"),
        )
        .arg(
            Arg::new("command")
                .long("command")
                .value_name("COMMAND")
                .help("Command to execute on remote system (requires active connection)")
                .conflicts_with("remoteId"),
        );

    let matches = app.get_matches();

    // Test rendezvous server and NAT type
    librustdesk::common::test_rendezvous_server();
    librustdesk::common::test_nat_type();

    if let Some(remote_id) = matches.get_one::<String>("remoteId") {
        // Connection mode
        let id_server = matches.get_one::<String>("idServer").map(|s| s.to_owned());
        let relay_server = matches.get_one::<String>("relayServer").map(|s| s.to_owned());
        let key = matches
            .get_one::<String>("key")
            .map(|s| s.to_owned())
            .unwrap_or_default();
        let password = matches.get_one::<String>("password").map(|s| s.to_owned());

        log::info!("RustDesk CLI - Connecting to remote ID: {}", remote_id);
        if let Some(ref server) = id_server {
            log::info!("Using ID server: {}", server);
        }
        if let Some(ref relay) = relay_server {
            log::info!("Using relay server: {}", relay);
        }

        match cli_controller::connect(
            remote_id,
            id_server,
            relay_server,
            key,
            password,
        ) {
            Ok(_) => {
                log::info!("Connection closed successfully");
            }
            Err(e) => {
                log::error!("Connection failed: {}", e);
                std::process::exit(1);
            }
        }
    } else if let Some(command) = matches.get_one::<String>("command") {
        // Command execution mode
        log::info!("Executing command: {}", command);
        match cli_controller::execute_command(command) {
            Ok(output) => {
                println!("{}", output);
            }
            Err(e) => {
                log::error!("Command execution failed: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Error: Either --remoteId or --command must be specified");
        std::process::exit(1);
    }

    // Clean up
    librustdesk::common::global_clean();
}
