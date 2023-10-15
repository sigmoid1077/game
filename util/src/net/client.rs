use crate::{TcpStreamComponent, Packet};
use std::{
    net::{Shutdown, TcpStream, SocketAddr}, 
    io::{Read, Write}
};
use bevy::{
    app::{App, Plugin, Update}, 
    core::Name,
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::QuerySingleError,
        system::{Commands, Query}
    }
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ConnectEvent>()
            .add_event::<DisconnectEvent>()
            .add_event::<ServerUnboundEvent>()
            .add_event::<SendPacketToServerEvent>()
            .add_event::<RecievedPacketFromServerEvent>()
            .add_systems(Update, (
                connect_event_system,
                disconnect_event_system,
                send_packet_to_server_event_system,
                listen_for_server_packets_system
            ));
    }
}

#[derive(Event)]
pub struct ConnectEvent {
    socket_addr: SocketAddr
}

impl ConnectEvent {
    pub fn new(socket_addr: SocketAddr) -> Self {
        Self { socket_addr }
    }
}

#[derive(Event)]
pub struct DisconnectEvent;

#[derive(Event)]
pub struct ServerUnboundEvent;

#[derive(Event)]
pub struct SendPacketToServerEvent {
    pub packet: Packet
}

impl SendPacketToServerEvent {
    pub fn new(packet: Packet) -> Self {
        Self { packet }
    }
}

#[derive(Event)]
pub struct RecievedPacketFromServerEvent {
    pub packet: Packet
}

impl RecievedPacketFromServerEvent {
    fn new(packet: Packet) -> Self {
        Self { packet }
    }
}

fn connect_event_system(
    mut commands: Commands,
    mut connect_events: EventReader<ConnectEvent>,
    tcp_stream_component_query: Query<&TcpStreamComponent>,
) {
    for _ in connect_events.iter() {
        if let Err(QuerySingleError::NoEntities(_)) = tcp_stream_component_query.get_single() {
            let tcp_stream = TcpStream::connect("127.0.0.1:2560").unwrap();
            tcp_stream.set_nonblocking(true).unwrap();
            commands.spawn((TcpStreamComponent::new(tcp_stream), Name::new("TcpStream")));
        }
    }
}

fn disconnect_event_system(
    mut commands: Commands,
    mut disconnect_events: EventReader<DisconnectEvent>,
    tcp_stream_entity_and_tcp_stream_component_query: Query<(Entity, &TcpStreamComponent)>
) {
    for _ in disconnect_events.iter() {
        if let Ok((tcp_stream_entity, tcp_stream_component)) = tcp_stream_entity_and_tcp_stream_component_query.get_single() {
            tcp_stream_component.tcp_stream.shutdown(Shutdown::Both).unwrap();
            commands.entity(tcp_stream_entity).despawn();
        }
    }
}

fn send_packet_to_server_event_system(
    mut send_packet_to_server_events: EventReader<SendPacketToServerEvent>,
    mut tcp_stream_component_query: Query<&mut TcpStreamComponent>
) {
    for send_packet_to_server_event in send_packet_to_server_events.iter() {
        if let Ok(mut tcp_stream_component) = tcp_stream_component_query.get_single_mut() {
            tcp_stream_component.tcp_stream.write(&bincode::serialize(&send_packet_to_server_event.packet).unwrap()).unwrap();
        }
    }
}

fn listen_for_server_packets_system(
    mut commands: Commands,
    mut received_packet_from_server_event: EventWriter<RecievedPacketFromServerEvent>,
    mut server_unbound_event: EventWriter<ServerUnboundEvent>,
    mut tcp_stream_entity_and_tcp_stream_component_query: Query<(Entity, &mut TcpStreamComponent)>
) {
    if let Ok((tcp_stream_entity, mut tcp_stream_component)) = tcp_stream_entity_and_tcp_stream_component_query.get_single_mut() {
        let mut buffer = [0; 1024];
        match tcp_stream_component.tcp_stream.read(&mut buffer) {
            Ok(0) => {
                // send server (and client?) data with event
                server_unbound_event.send(ServerUnboundEvent);
                tcp_stream_component.tcp_stream.shutdown(Shutdown::Both).unwrap();
                commands.entity(tcp_stream_entity);
            },
            Ok(packet_length) => received_packet_from_server_event.send(RecievedPacketFromServerEvent::new(bincode::deserialize(&buffer[0..packet_length]).unwrap())),
            Err(_) => ()
        }
    }
}
