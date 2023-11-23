use bevy::ecs::{event::{EventIterator, EventReader, EventWriter}, system::SystemParam};
use crate::{Packet, server::event};

#[derive(SystemParam)]
pub struct ServerSystemParam<'w, 's, Sp: Packet, Rp: Packet> {
    bind_event: EventWriter<'w, event::write::BindEvent>,
    unbind_event: EventWriter<'w, event::write::UnbindEvent>,
    send_packet_to_all_clients_event: EventWriter<'w, event::write::SendPacketToAllClients<Sp>>,
    recieved_packet_from_client_events: EventReader<'w, 's, event::read::RecievedPacketFromClientEvent<Rp>>
}

impl<'w, 's, Sp: Packet, Rp: Packet> ServerSystemParam<'w, 's, Sp, Rp> {
    pub fn bind(&mut self, port: u16) {
        self.bind_event.send(event::write::BindEvent(port));
    }

    pub fn unbind(&mut self) {
        self.unbind_event.send(event::write::UnbindEvent);
    }

    pub fn send_packet_to_client(&mut self) {
        unimplemented!()
    }

    pub fn send_packet_to_all_clients(&mut self, packet: Sp) {
        self.send_packet_to_all_clients_event.send(event::write::SendPacketToAllClients(packet));
    }

    pub fn recieved_packets(&mut self) -> EventIterator<'_, event::read::RecievedPacketFromClientEvent<Rp>> {
        self.recieved_packet_from_client_events.read()
    }
}
