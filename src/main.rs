#[macro_use]
extern crate rocket;

use liquidation_monitor::{
    cache,
    cache::{AnchorCache, Borrowers},
    event::handler,
    observer::client::ObserverClient,
};
use rocket::State;
use rust_decimal::Decimal;
use tokio::sync::mpsc;
use tracing::{error, info};
use tungstenite::error::{Error::Protocol, ProtocolError::ResetWithoutClosingHandshake};

#[get("/borrowers")]
async fn borrowers(borrowers: &State<Borrowers>) -> String {
    cache::cached_borrowers(borrowers.read().await.clone()).unwrap()
}

#[get("/liqs?<beth_price>")]
async fn liqs(borrowers: &State<Borrowers>, beth_price: Option<usize>) -> String {
    let beth_price = Decimal::new(beth_price.unwrap_or(2_800_000_000) as i64, 6); // TODO: Fetch current bETH price if none is provided?
    cache::cached_liquidations(borrowers.read().await.clone(), &beth_price).unwrap()
}

#[rocket::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting the liquidation-monitor...");
    let mut client = ObserverClient::default();
    let (tx, rx) = mpsc::channel(1000);
    let cache = AnchorCache::new();
    cache
        .seed_borrowers("borrowers_seed.json")
        .await
        .expect("Error seeding borrowers data");
    cache.init_listener(rx);

    info!("Launching API server...");
    tokio::spawn(
        rocket::build()
            .mount("/api", routes![borrowers, liqs])
            .manage(cache.borrowers)
            .launch(),
    );

    info!("Listening to WebSocket...");
    loop {
        let res = client.socket.read_message();
        let tx = tx.clone();

        match res {
            Ok(msg) => {
                tokio::spawn(handler::handle_msg(msg, tx));
            }
            Err(Protocol(ResetWithoutClosingHandshake)) => {
                error!("ResetWithoutClosingHandshake error! Restarting Observer Client...");
                client = ObserverClient::default();
            }
            Err(_) => {}
        }
    }
}
