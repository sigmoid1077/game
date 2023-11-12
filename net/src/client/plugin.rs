use bevy::app::{App, Plugin, Update};
use crate::{
    client::{event::*, system::*},
    util::{SendingPacket, RecievingPacket}
};
use std::marker::PhantomData;

pub struct ClientPlugin<Sp: SendingPacket, Rp: RecievingPacket>(pub PhantomData<Sp>, pub PhantomData<Rp>);

impl<Sp: SendingPacket, Rp: RecievingPacket> Plugin for ClientPlugin<Sp, Rp> {
    fn build(&self, app: &mut App) {
        app
            .add_event::<write::ConnectEvent>()
            .add_event::<write::DisconnectEvent>()
            .add_event::<write::SendPacketEvent<Sp>>()
            .add_event::<read::ServerDisconnectedEvent>()
            .add_event::<read::RecievedPacket<Rp>>()
            .add_systems(Update, (
                read_connect_event_system,
                read_disconnect_event_system,
                write_send_packet_event_system::<Sp>,
                write_server_disconnected_event_and_recieved_packet_event_system::<Rp>
            ));
    }
}