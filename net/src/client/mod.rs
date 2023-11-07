pub mod event;
mod system;

// add support for generic packet types
// add support for custom serializers

use bevy::app::{App, Plugin, Update};
use crate::{SendingPacket, RecievingPacket};
use std::marker::PhantomData;

pub struct ClientPlugin<Sp: SendingPacket, Rp: RecievingPacket>(pub PhantomData<Sp>, pub PhantomData<Rp>);

impl<Sp: SendingPacket, Rp: RecievingPacket> Plugin for ClientPlugin<Sp, Rp> {
    fn build(&self, app: &mut App) {
        app
            .add_event::<event::write::ConnectEvent>()
            .add_event::<event::write::DisconnectEvent>()
            .add_event::<event::write::SendPacketEvent<Sp>>()
            .add_event::<event::read::ServerDisconnectedEvent>()
            .add_event::<event::read::RecievedPacket<Rp>>()
            .add_systems(Update, (
                system::read_connect_event_system,
                system::read_disconnect_event_system,
                system::write_send_packet_event_system::<Sp>,
                system::write_server_disconnected_event_and_recieved_packet_event_system::<Rp>
            ));
    }
}
