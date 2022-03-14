mod wallet_manager;
mod blockchain_manager;
mod mempool_manager;
use std::env;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::{thread, time::Duration};
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use crate::wallet_manager::create_wallet;
use crate::blockchain_manager::{Block, generate_block, write_block};
use crate::mempool_manager::{new_txn};

#[tokio::main]
/// Master-node starter
async fn main() {
    //!
    //! Binding server to 127.0.0.1:4736
    let addr = SocketAddr::from(([127, 0, 0, 1], 4736));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(parse_hash))
    });

    for arg in env::args() {
        if arg == "create_wallet".to_string() {
            let wallet = create_wallet();
            println!("Public key: {}", wallet.public_key);
            println!("Private key: {}", wallet.private_key);
        }
    }

    thread::spawn(|| {
        loop {
            run_node();
        }
    });

    let server = Server::bind(&addr)
        .serve(make_svc);

    if let Err(_) = server.await {
        println!("Server error!");
    }
}

fn run_node() {
    write_block(generate_block());
    thread::sleep(Duration::from_millis(1000));
}

/// Parse Hash from HTTP Query
async fn parse_hash(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    //!
    //! ```no_run
    //! hash = from_public_key:encoded_message:to_public_key
    //! ```
    let request_uri = req.uri().to_string();
    let query = request_uri.split("=");
    let vec = query.collect::<Vec<&str>>();

    // Validate Hash Stock in Query
    return if vec.len() == 2  {
        let hash = vec[1];
        let check_hash = hash.split(":").collect::<Vec<&str>>();

        // Validate Hashes Standard (40 chars)
        if check_hash.len() == 3 && check_hash[0].len() == 40 && check_hash[2].len() == 40 {
            // Writing Block to Blockchain
            new_txn(hash);

            Ok(
                Response::builder()
                    .status(StatusCode::OK)
                    .body("200 OK".into())
                    .unwrap()
            )
        } else {
            Ok(
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("402 BAD REQUEST".into())
                    .unwrap()
            )
        }
    } else {
        Ok(
            Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body("403 FORBIDDEN".into())
                .unwrap()
        )
    }
}