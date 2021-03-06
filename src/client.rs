mod announcement;
mod api_key;
mod chat;
mod execution;
mod funding;
mod global_notification;
mod instrument;
mod insurance;
mod leaderboard;
mod liquidation;
mod order;
mod order_book;
mod position;
mod quote;
mod settlement;
mod trade;
mod user;
mod user_event;
pub mod websocket;

use crate::transport::Transport;

#[derive(Clone)]
pub struct BitMEX {
    pub(crate) transport: Transport,
}

impl Default for BitMEX {
    fn default() -> Self {
        Self::new()
    }
}

impl BitMEX {
    pub fn new() -> Self {
        BitMEX { transport: Transport::new() }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        BitMEX {
            transport: Transport::with_credential(api_key, api_secret),
        }
    }
}
