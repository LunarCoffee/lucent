#![feature(box_syntax)]
#![feature(is_symlink)]
#![feature(slice_as_chunks)]
#![feature(slice_group_by)]

use std::env;

use async_std::{process, sync::Arc};

use crate::server::{config::Config, file_server::{FileServer, FileServerStartError::*}, Server};

mod consts;
mod http;
mod log;
mod server;
mod util;

#[async_std::main]
async fn main() {
    // The only argument taken is mandatory, the path to the config file.
    let mut args = env::args();
    if args.len() != 2 {
        println!("usage: {} <config path>", args.next().unwrap());
        process::exit(1);
    }

    log::info(format!("lucent v{}", consts::SERVER_VERSION));
    let config = Config::load(&args.nth(1).unwrap()).await
        .unwrap_or_else(|| log::fatal("configuration file invalid or missing required options"));

    log::fatal(match FileServer::new(config).await {
        // Register a signal handler for graceful shutdowns and start the server.
        Ok(server) => {
            let server = Arc::new(server);
            let server_clone = Arc::clone(&server);
            if let Err(_) = ctrlc::set_handler(move || server_clone.stop()) {
                log::warn("failed to attach signal handler for graceful shutdown");
            }
            return server.start();
        }
        // Initialization failed, here's why.
        Err(InvalidFileRoot) => "file directory invalid",
        Err(InvalidTemplates) => "template directory invalid or missing files",
        Err(AddressInUse) => "that address is in use",
        Err(AddressUnavailable) => "that address is unavailable",
        Err(CannotBindAddress) => "cannot bind to that address",
        Err(TlsCertNotFound) => "cannot find TLS certificate file",
        Err(TlsKeyNotFound) => "cannot find RSA private key file",
        Err(TlsInvalidCert) => "that TLS certificate is invalid",
        _ => "that RSA private key is invalid",
    });
}
