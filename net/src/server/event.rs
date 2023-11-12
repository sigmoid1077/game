use bevy::ecs::event::Event;
use crate::util::{RecievingPacket, SendingPacket};

pub mod write {
    use super::*;
    
    #[derive(Event)]
    pub struct BindEvent(pub u16);

    #[derive(Event)]
    pub struct UnbindEvent;

    #[derive(Event)]
    pub struct SendPacketToClient<Sp: SendingPacket>(/* client */ pub Sp);

    #[derive(Event)]
    pub struct SendPacketToAllClients<Sp: SendingPacket>(pub Sp);
}

pub mod read {
    use super::*;

    #[derive(Event)]
    pub struct ClientConnectedEvent(/* client */);

    #[derive(Event)]
    pub struct ClientDisconnectedEvent(/* client */);

    #[derive(Event)]
    pub struct RecievedPacketFromClientEvent<Rp: RecievingPacket>(/* client */ pub Rp);
}
