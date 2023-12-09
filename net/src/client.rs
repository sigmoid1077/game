use bevy::{ecs::system::SystemParam, prelude::*};
use crate::{ClientComponent, Packet};
use std::{io::{Read, Write}, marker::PhantomData, net::{SocketAddr, Shutdown, TcpStream}};

pub struct ClientPlugin<SendPacket: Packet, RecvPacket: Packet>(PhantomData<SendPacket>, PhantomData<RecvPacket>);

impl<SendPacket: Packet, RecvPacket: Packet> Plugin for ClientPlugin<SendPacket, RecvPacket> {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ConnectEvent>()
            .add_event::<DisconnectEvent>()
            .add_event::<RecvEvent<RecvPacket>>()
            .add_event::<SendEvent<SendPacket>>()
            .add_event::<UnboundEvent>()
            .add_systems(Update, (
                read_connect_event,
                read_disconnect_event,
                write_send_event::<SendPacket>,
                write_unbind_and_recv_events::<RecvPacket>
            ));
    }
}

#[derive(SystemParam)]
pub struct ClientSystemParam<'w, 's, SendPacket: Packet, RecvPacket: Packet> {
    connect_event: EventWriter<'w, ConnectEvent>,
    disconnect_event: EventWriter<'w, DisconnectEvent>,
    recv_events: EventReader<'w, 's, RecvEvent<RecvPacket>>,
    send_event: EventWriter<'w, SendEvent<SendPacket>>
}

impl<'w, 's, SendPacket: Packet, RecvPacket: Packet> ClientSystemParam<'w, 's, SendPacket, RecvPacket> {
    pub fn connect(&mut self, socket_addr: SocketAddr) {
        self.connect_event.send(ConnectEvent(socket_addr));
    }

    pub fn disconnect(&mut self) {
        self.disconnect_event.send(DisconnectEvent);
    }
    
    pub fn recieved_packets(&mut self) -> impl Iterator<Item=&RecvPacket> {
        self.recv_events.read().map(|recieved_packet_event| &recieved_packet_event.0)
    }
    
    pub fn send_packet(&mut self, packet: SendPacket) {
        self.send_event.send(SendEvent(packet));
    }
}

#[derive(Event)]
pub(crate) struct ConnectEvent(pub(crate) SocketAddr);

#[derive(Event)]
pub(crate) struct DisconnectEvent;

#[derive(Event)]
pub(crate) struct SendEvent<SendPacket: Packet>(pub(crate) SendPacket);

#[derive(Event)]
pub(crate) struct UnboundEvent;

#[derive(Event)]
pub(crate) struct RecvEvent<RecvPacket: Packet>(pub(crate) RecvPacket);

pub(crate) fn read_connect_event(
    mut commands: Commands,
    mut connect_events: EventReader<ConnectEvent>,
    client_component_query: Query<&ClientComponent>
) {
    for connect_event in connect_events.read() {
        if client_component_query.iter().count() == 0 {
            let tcp_stream = TcpStream::connect(connect_event.0).unwrap();
            tcp_stream.set_nonblocking(true).unwrap();
            commands.spawn(ClientComponent(tcp_stream));
        }
    }
}

pub(crate) fn read_disconnect_event(
    mut commands: Commands,
    mut disconect_events: EventReader<DisconnectEvent>,
    entity_with_client_component_query: Query<Entity, With<ClientComponent>>,
    client_component_query: Query<&ClientComponent>
) {
    for _disconnect_event in disconect_events.read() {
        if let Ok(entity_with_client_component) = entity_with_client_component_query.get_single() {
            if let Ok(client_component) = client_component_query.get_component::<ClientComponent>(entity_with_client_component) {
                client_component.0.shutdown(Shutdown::Both).unwrap();
                commands.entity(entity_with_client_component).despawn();
            }
        }
    }
}

pub(crate) fn write_send_event<SendPacket: Packet>(
    mut send_packet_events: EventReader<SendEvent<SendPacket>>,
    entity_with_client_component_query: Query<Entity, With<ClientComponent>>,
    mut client_component_query: Query<&mut ClientComponent>
) {
    for send_packet_event in send_packet_events.read() {
        if let Ok(entity_with_client_component) = entity_with_client_component_query.get_single() {
            if let Ok(mut client_component) = client_component_query.get_component_mut::<ClientComponent>(entity_with_client_component) {
                client_component.0.write_all(&send_packet_event.0.serialize_packet()).unwrap();
            }
        }
    }
}

pub(crate) fn write_unbind_and_recv_events<RecvPacket: Packet>(
    mut commands: Commands,
    entity_with_client_component_query: Query<Entity, With<ClientComponent>>,
    mut mut_client_component_query: Query<&mut ClientComponent>,
    mut recieved_packet_events: EventWriter<RecvEvent<RecvPacket>>,
    mut server_disconnected_events: EventWriter<UnboundEvent>,
) {
    if let Ok(entity_with_client_component) = entity_with_client_component_query.get_single() {
        if let Ok(mut client_component) = mut_client_component_query.get_component_mut::<ClientComponent>(entity_with_client_component) {
            let mut buf = [0; 1024];
            
            match client_component.0.read(&mut buf) {
                Ok(0) => {
                    server_disconnected_events.send(UnboundEvent);
                    client_component.0.shutdown(Shutdown::Both).unwrap();
                    commands.entity(entity_with_client_component).despawn();
                },
                Ok(packet_length) => recieved_packet_events.send(RecvEvent(RecvPacket::deserialize_packet(&buf[0..packet_length]))),
                _ => ()
            }
        }
    }
}
