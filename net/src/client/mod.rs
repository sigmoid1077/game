mod event;
mod system;

// add support for generic packet types
// add support for custom serializers

use bevy::app::{App, Plugin, Update};
use crate::Packet;
use std::marker::PhantomData;

pub struct ClientPlugin<P: Packet>(PhantomData<P>);

impl<P: Packet> Plugin for ClientPlugin<P> {
    fn build(&self, app: &mut App) {
        app
            .add_event::<event::write::ConnectEvent>()
            .add_event::<event::write::DisconnectEvent>()
            .add_event::<event::write::SendPacketEvent<P>>()
            .add_event::<event::read::ServerDisconnectedEvent>()
            .add_event::<event::read::RecievedPacket<P>>()
            .add_systems(Update, (
                system::read_connect_event_system,
                system::read_disconnect_event_system,
                system::write_send_packet_event_system::<P>,
                system::write_server_disconnected_event_and_recieved_packet_event_system::<P>
            ));
    }
}
