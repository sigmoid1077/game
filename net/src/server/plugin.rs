use bevy::app::{App, Plugin, Update};
use crate::{
    server::{event::*, system::*},
    util::{RecievingPacket, SendingPacket}
};
use std::marker::PhantomData;

pub struct ServerPlugin<Sp: SendingPacket, Rp: RecievingPacket>(pub PhantomData<Sp>, pub PhantomData<Rp>);

impl<Sp: SendingPacket, Rp: RecievingPacket> Plugin for ServerPlugin<Sp, Rp> {
    fn build(&self, app: &mut App) {
        app
            .add_event::<write::BindEvent>()
            .add_event::<write::UnbindEvent>()
            .add_event::<write::SendPacketToClient<Sp>>()
            .add_event::<write::SendPacketToAllClients<Sp>>()
            .add_event::<read::ClientConnectedEvent>()
            .add_event::<read::ClientDisconnectedEvent>()
            .add_event::<read::RecievedPacketFromClientEvent<Rp>>()
            .add_systems(Update, (
                read_bind_event_system,
                read_unbind_event_system,
                read_send_packet_to_client_event_system::<Sp>,
                read_send_packet_to_all_clients_event_system::<Sp>,
                write_client_connected_event_system,
                write_client_disconnected_event_and_recieved_packet_from_client_event_system::<Rp>
            ));
    }
}
