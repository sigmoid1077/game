use bevy::ecs::event::Event;
use crate::Packet;

pub mod write {
    use super::*;
    
    #[derive(Event)]
    pub struct BindEvent(pub u16);

    #[derive(Event)]
    pub struct UnbindEvent;

    #[derive(Event)]
    pub struct SendPacketToClient<Sp: Packet>(/* client */ pub Sp);

    #[derive(Event)]
    pub struct SendPacketToAllClients<Sp: Packet>(pub Sp);
}

pub mod read {
    use super::*;

    #[derive(Event)]
    pub struct ClientConnectedEvent(/* client */);

    #[derive(Event)]
    pub struct ClientDisconnectedEvent(/* client */);

    #[derive(Event)]
    pub struct RecievedPacketFromClientEvent<Rp: Packet>(/* client */ pub Rp);
}
