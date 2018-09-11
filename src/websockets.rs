use model::*;
use errors::*;
use url::Url;
use serde_json::from_str;

use tungstenite::connect;
use tungstenite::protocol::WebSocket;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;

static WEBSOCKET_URL: &'static str = "wss://real.okex.com:10441/websocket";

static PARTIAL_ORDERBOOK : &'static str = "lastUpdateId";

pub trait MarketEventHandler {
    fn partial_orderbook_handler(&self, order_book: &OrderBook);
}

#[derive(Default)]
pub struct WebSockets {
    socket: Option<(WebSocket<AutoStream>, Response)>,
    market_handler: Option<Box<MarketEventHandler>>
}

impl WebSockets {
    pub fn new() -> WebSockets {
        WebSockets {
            socket: None,
            market_handler: None
        }
    }

    pub fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}{}", WEBSOCKET_URL, endpoint);
        let url = Url::parse(&wss)?;

        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => {
                bail!(format!("Error during handshake {}", e));
            }
        }
    }

    pub fn add_market_handler<H>(&mut self, handler: H)
    where
        H: MarketEventHandler + 'static,
    {
        self.market_handler = Some(Box::new(handler));
    }

    pub fn event_loop(&mut self) {
        loop {
            if let Some(ref mut socket) = self.socket {
                let msg: String = socket.0.read_message().unwrap().into_text().unwrap();

                if msg.find(PARTIAL_ORDERBOOK) != None {
                    let partial_orderbook: OrderBook = from_str(msg.as_str()).unwrap();

                    if let Some(ref h) = self.market_handler {
                        h.partial_orderbook_handler(&partial_orderbook);
                    }
                }
            }
        }
    }    
}