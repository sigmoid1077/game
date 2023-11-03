// add support for generic packet types

use bevy::ecs::event::Event;
use super::Packet;

pub mod write {
    use super::*;
    
    #[derive(Event)]
    pub struct BindEvent(pub u16);

    #[derive(Event)]
    pub struct UnbindEvent;

    #[derive(Event)]
    pub struct SendPacketToClient<P: Packet>(/* client */ pub P);

    #[derive(Event)]
    pub struct SendPacketToAllClients<P: Packet>(pub P);
}

pub mod read {
    use super::*;

    #[derive(Event)]
    pub struct ClientConnectedEvent(/* client */);

    #[derive(Event)]
    pub struct ClientDisconnectedEvent(/* client */);

    #[derive(Event)]
    pub struct RecievedPacketFromClientEvent<P: Packet>(/* client */ pub P);
}
