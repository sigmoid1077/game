use bevy::ecs::event::Event;
use crate::Packet;
use std::net::SocketAddr;

pub(crate) mod write {
    use super::*;

    #[derive(Event)]
    pub(crate) struct ConnectEvent(pub SocketAddr);

    #[derive(Event)]
    pub(crate) struct DisconnectEvent;

    #[derive(Event)]
    pub(crate) struct SendPacketEvent<Sp: Packet>(pub Sp);
}

pub(crate) mod read {
    use super::*;

    #[derive(Event)]
    pub(crate) struct ServerDisconnectedEvent;

    #[derive(Event)]
    pub(crate) struct RecievedPacket<Rp: Packet>(pub Rp);
}
