use std::thread;

use anyhow::Error;
use tiny_http::{Method, Response, Server};

const METHOD_NOT_FOUND: u16 = 404;
const METHOD_NOT_ALLOWED: u16 = 405;

/// Start a simple HTTP server on a new thread at the given address and expose Prometheus metrics.
/// This server is intended to only be queried synchronously as it blocks upon receiving
/// each request.
pub fn serve(addr: &str) -> Result<(), Error> {
    let server = match Server::http(addr) {
        Ok(s) => s,
        Err(_) => return Err(Error::msg("error creating metrics server")),
    };
    println!("Exposing Prometheus metrics server: {}", addr);

    // Handle requests in a new thread so we can process in the background.
    thread::spawn(move || {
        loop {
            // Blocks until the next request is received.
            let req = match server.recv() {
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

            // Write the Prometheus metrics.
            let res = Response::from_string("metrics");
            if let Err(e) = req.respond(res) {
                eprintln!("{}", e);
            };
        }
    });

    Ok(())
}
