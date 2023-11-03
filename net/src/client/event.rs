// add support for generic packet types

use bevy::ecs::event::Event;
use super::Packet;
use std::net::SocketAddr;

pub mod write {
    use super::*;

    // add field for server ip
    #[derive(Event)]
    pub struct ConnectEvent(pub SocketAddr);

    #[derive(Event)]
    pub struct DisconnectEvent;

    #[derive(Event)]
    pub struct SendPacketEvent<P: Packet>(pub P);
}

pub mod read {
    use super::*;

    #[derive(Event)]
    pub struct ServerDisconnectedEvent;

    #[derive(Event)]
    pub struct RecievedPacket<P: Packet>(pub P);
}
