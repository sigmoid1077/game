use bevy::app::{App, Plugin, Update};
use crate::{Packet, server::{event, system}};
use std::marker::PhantomData;

pub struct ServerPlugin<const BUFFER_SIZE: usize, Sp: Packet, Rp: Packet>(pub PhantomData<Sp>, pub PhantomData<Rp>);

impl<const BUFFER_SIZE: usize, Sp: Packet, Rp: Packet> Plugin for ServerPlugin<BUFFER_SIZE, Sp, Rp> {
    fn build(&self, app: &mut App) {
        if BUFFER_SIZE < 128 || BUFFER_SIZE > 16384 {
            panic!("Server network buffer size out of bounds.");
        }

        app
            .add_event::<event::write::BindEvent>()
            .add_event::<event::write::UnbindEvent>()
            .add_event::<event::write::SendPacketToClient<Sp>>()
            .add_event::<event::write::SendPacketToAllClients<Sp>>()
            .add_event::<event::read::ClientConnectedEvent>()
            .add_event::<event::read::ClientDisconnectedEvent>()
            .add_event::<event::read::RecievedPacketFromClientEvent<Rp>>()
            .add_systems(Update, (
                system::read_bind_event_system,
                system::read_unbind_event_system,
                system::read_send_packet_to_client_event_system::<Sp>,
                system::read_send_packet_to_all_clients_event_system::<Sp>,
                system::write_client_connected_event_system,
                system::write_client_disconnected_event_and_recieved_packet_from_client_event_system::<BUFFER_SIZE, Rp>
            ));
    }
}
