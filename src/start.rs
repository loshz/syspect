use std::{thread, time};

use anyhow::Error;
use clap::ArgMatches;
use tiny_http::{Method, Response, Server};

use crate::config;

const METHOD_NOT_FOUND: u16 = 404;
const METHOD_NOT_ALLOWED: u16 = 405;

/// The command name.
pub const COMMAND_NAME: &str = "start";

pub fn run(args: &ArgMatches) -> Result<(), Error> {
    println!(
        "Starting service, version: {} {}",
        crate::PKG_NAME,
        crate::PKG_VERSION
    );

    let path = match args.value_of("config") {
        Some(path) => path,
        _ => return Err(Error::msg("config not specified".to_owned())),
    };
    println!("Using config: {}", path);

    let c = config::from_file(path)?;

    // Expose the Prometheus metrics.
    let addr = c.metrics_address();
    let server = match Server::http(addr.as_str()) {
        Ok(s) => s,
        Err(_) => return Err(Error::msg("error creating metrics server")),
    };
    println!("Exposing Prometheus metrics server: {}", addr);
    thread::spawn(move || serve_metrics(server));

    // simulate work
    let ten = time::Duration::from_secs(600);
    thread::sleep(ten);

    Ok(())
}

fn serve_metrics(srv: Server) {
    loop {
        // Blocks until the next request is received.
        let req = match srv.recv() {
            Ok(req) => req,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };

        // Only reponsd to GET requests.
        if req.method() != &Method::Get {
            let res = Response::empty(METHOD_NOT_ALLOWED);
            if let Err(e) = req.respond(res) {
                eprintln!("{}", e);
            };
            continue;
        }

        // Only serve the /metrics path.
        if !req.url().starts_with("/metrics") {
            let res = Response::empty(METHOD_NOT_FOUND);
            if let Err(e) = req.respond(res) {
                eprintln!("{}", e);
            };
            continue;
        }

        let res = Response::from_string("metrics");
        if let Err(e) = req.respond(res) {
            eprintln!("{}", e);
        };
    }
}
