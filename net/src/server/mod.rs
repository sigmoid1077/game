// #[cfg(not(target_arch = "wasm32"))] for this entire module
// add support for generic packet types
// add support for custom serializers

mod event;
mod system;

use bevy::app::{App, Plugin, Update};
use crate::Packet;
use std::marker::PhantomData;

pub struct ServerPlugin<P: Packet>(PhantomData<P>);

impl<P: Packet> Plugin for ServerPlugin<P> {
    fn build(&self, app: &mut App) {
        app
            .add_event::<event::write::BindEvent>()
            .add_event::<event::write::UnbindEvent>()
            .add_event::<event::write::SendPacketToClient<P>>()
            .add_event::<event::write::SendPacketToAllClients<P>>()
            .add_event::<event::read::ClientConnectedEvent>()
            .add_event::<event::read::ClientDisconnectedEvent>()
            .add_event::<event::read::RecievedPacketFromClientEvent<P>>()
            .add_systems(Update, (
                system::read_bind_event_system,
                system::read_unbind_event_system,
                system::read_send_packet_to_client_event_system::<P>,
                system::read_send_packet_to_all_clients_event_system::<P>,
                system::write_client_connected_event_system,
                system::write_client_disconnected_event_and_recieved_packet_from_client_event_system::<P>
            ));
    }
}
