use bevy::ecs::event::Event;
use crate::{RecievingPacket, SendingPacket};
use std::net::SocketAddr;

pub(crate) mod write {
    use super::*;

    #[derive(Event)]
    pub struct ConnectEvent(pub SocketAddr);

    #[derive(Event)]
    pub struct DisconnectEvent;

    #[derive(Event)]
    pub struct SendPacketEvent<Sp: SendingPacket>(pub Sp);
}

pub(crate) mod read {
    use super::*;

    #[derive(Event)]
    pub struct ServerDisconnectedEvent;

    #[derive(Event)]
    pub struct RecievedPacket<Rp: RecievingPacket>(pub Rp);
}
