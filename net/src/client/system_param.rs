use bevy::ecs::{
    event::{EventReader, EventWriter, EventIterator},
    system::SystemParam
};
use crate::{client::event, SendingPacket, RecievingPacket};
use std::net::SocketAddr;

#[derive(SystemParam)]
pub struct ClientSystemParam<'w, 's, Sp: SendingPacket, Rp: RecievingPacket> {
    connect_event: EventWriter<'w, event::write::ConnectEvent>,
    disconnect_event: EventWriter<'w, event::write::DisconnectEvent>,
    recieved_packet_events: EventReader<'w, 's, event::read::RecievedPacket<Rp>>,
    send_packet_event: EventWriter<'w, event::write::SendPacketEvent<Sp>>
}

impl<'w, 's, Sp: SendingPacket, Rp: RecievingPacket> ClientSystemParam<'w, 's, Sp, Rp> {
    pub fn connect(&mut self, socket_addr: SocketAddr) {
        self.connect_event.send(event::write::ConnectEvent(socket_addr));
    }

    pub fn disconnect(&mut self) {
        self.disconnect_event.send(event::write::DisconnectEvent);
    }
    
    pub fn recieved_packets(&mut self) -> EventIterator<'_, event::read::RecievedPacket<Rp>> {
        self.recieved_packet_events.read()
    }
    
    pub fn send_packet(&mut self, packet: Sp) {
        self.send_packet_event.send(event::write::SendPacketEvent(packet));
    }
}