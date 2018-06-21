extern crate crossbeam_channel;

mod cb_ping;
mod cb_pingpong;
mod ping;
mod pingpong;

pub use cb_ping::cb_ping;
pub use cb_pingpong::cb_pingpong;
pub use ping::ping;
pub use pingpong::pingpong;

pub enum Kind {
    Ping,
    PingPong,
    CbPing,
    CbPingPong,
}

impl Kind {
    pub fn try_from(s: &str) -> Result<Kind, &'static str> {
        match s.to_ascii_lowercase().as_ref() {
            "ping" => Ok(Kind::Ping),
            "pong" => Ok(Kind::PingPong),
            "cbping" => Ok(Kind::CbPing),
            "cbpingpong" => Ok(Kind::CbPingPong),
            _ => Err("must be one of Ping, Pong, CbPing, CbPingPong"),
        }
    }
}
