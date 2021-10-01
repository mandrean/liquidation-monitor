use serde::{Deserialize, Serialize};
use tracing::info;
use tungstenite::client::AutoStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
struct Id(pub String);

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
struct SubscribeMessage {
    pub subscribe: SubscriptionEventType,
    pub chain_id: Id,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum SubscriptionEventType {
    NewBlock,
}

pub struct ObserverClient {
    pub socket: WebSocket<AutoStream>,
}

const TERRA_OBSERVER: &str = "wss://observer.terra.dev";

impl ObserverClient {
    pub fn default() -> ObserverClient {
        let mut socket = connect(Url::parse(TERRA_OBSERVER).unwrap())
            .expect("Can't connect to Terra Observer")
            .0;
        info!("Connected to Terra Observer");

        let msg = SubscribeMessage {
            subscribe: SubscriptionEventType::NewBlock,
            chain_id: Id("columbus-5".to_string()),
        };
        let json = serde_json::to_string(&msg).unwrap();
        socket
            .write_message(Message::Text(json))
            .expect("Can't subscribe to 'new_block' feed");
        info!("Subscribed to 'new_block' feed");

        ObserverClient { socket }
    }
}
