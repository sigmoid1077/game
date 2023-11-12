use bevy::ecs::event::Event;
use crate::util::{RecievingPacket, SendingPacket};
use std::net::SocketAddr;

pub mod write {
    use super::*;

    #[derive(Event)]
    pub struct ConnectEvent(pub SocketAddr);

    #[derive(Event)]
    pub struct DisconnectEvent;

    #[derive(Event)]
    pub struct SendPacketEvent<Sp: SendingPacket>(pub Sp);
}

pub mod read {
    use super::*;

    #[derive(Event)]
    pub struct ServerDisconnectedEvent;

    #[derive(Event)]
    pub struct RecievedPacket<Rp: RecievingPacket>(pub Rp);
}
