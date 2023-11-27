use bevy::app::{App, Plugin, Update};
use crate::{client::{event, system}, Packet};
use std::marker::PhantomData;

pub struct ClientPlugin<const BUFFER_SIZE: usize, Sp: Packet, Rp: Packet>(pub PhantomData<Sp>, pub PhantomData<Rp>);

impl<const BUFFER_SIZE: usize, Sp: Packet, Rp: Packet> Plugin for ClientPlugin<BUFFER_SIZE, Sp, Rp> {
    fn build(&self, app: &mut App) {
        if BUFFER_SIZE < 128 || BUFFER_SIZE > 16384 {
            panic!("Client network buffer size out of bounds.");
        }

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
                system::write_server_disconnected_event_and_recieved_packet_event_system::<BUFFER_SIZE, Rp>
            ));
    }
}
