use crate::{TcpListenerComponent, TcpStreamComponent, Packet};
use std::{
    net::{TcpListener, Shutdown}, 
    io::{Read, Write}
};
use bevy::{
    app::{App, Plugin, Update}, 
    core::Name,
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Commands, Query}
    }
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BindEvent>()
            .add_event::<UnbindEvent>()
            .add_event::<ClientConnectedEvent>()
            .add_event::<ClientDisconnectedEvent>()
            .add_event::<SendPacketToClientEvent>()
            .add_event::<SendPacketToAllClientsEvent>()
            .add_event::<RecievedPacketFromClientEvent>()
            .add_systems(Update, (
                bind_event_system,
                unbind_event_system,
                send_packet_to_client_event_system,
                send_packet_to_all_clients_event_system,
                listen_for_client_connections_system,
                listen_for_client_packets_system
            ));
    }
}

#[derive(Event)]
pub struct BindEvent {
    port: u16
}

impl BindEvent {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}

#[derive(Event)]
pub struct UnbindEvent;

#[derive(Event)]
pub struct ClientConnectedEvent;

#[derive(Event)]
pub struct ClientDisconnectedEvent;

#[derive(Event)]
pub struct SendPacketToClientEvent;

#[derive(Event)]
pub struct SendPacketToAllClientsEvent {
    pub packet: Packet
}

impl SendPacketToAllClientsEvent {
    pub fn new(packet: Packet) -> Self {
        Self { packet }
    }
}

#[derive(Event)]
pub struct RecievedPacketFromClientEvent {
    pub packet: Packet
}

impl RecievedPacketFromClientEvent {
    fn new(packet: Packet) -> Self {
        Self { packet }
    }
}

fn bind_event_system(
    mut bind_event: EventReader<BindEvent>,
    mut commands: Commands,
    tcp_listener_component_query: Query<&TcpListenerComponent>
) {
    for _ in bind_event.iter() {
        if tcp_listener_component_query.is_empty() {
            let tcp_listener = TcpListener::bind("127.0.0.1:2560").unwrap();
            tcp_listener.set_nonblocking(true).unwrap();
            commands.spawn((
                TcpListenerComponent::new(tcp_listener),
                Name::new("TcpListener")
            ));
        }
    }
}

fn unbind_event_system(
    mut commands: Commands,
    mut unbind_events: EventReader<UnbindEvent>,
    tcp_listener_entity_query: Query<Entity, With<TcpListenerComponent>>,
    tcp_stream_entity_and_tcp_stream_component_query: Query<(Entity, &TcpStreamComponent)>
) {
    for _ in unbind_events.iter() {
        for (tcp_stream_entity, tcp_stream_component) in tcp_stream_entity_and_tcp_stream_component_query.iter() {
            tcp_stream_component.tcp_stream.shutdown(Shutdown::Both).unwrap();
            commands.entity(tcp_stream_entity).despawn();
        }

        // despawn the server's tcp listener
        if let Ok(tcp_listener_entity) = tcp_listener_entity_query.get_single() {
            commands.entity(tcp_listener_entity).despawn();
        }
    }
}

fn send_packet_to_client_event_system(
    mut send_packet_to_client_events: EventReader<SendPacketToClientEvent>
) {
    for _ in send_packet_to_client_events.iter() {
       // find which way to filter through clients
       // will it be by ip?
       // will it be by some kind of id?
       // will it be by the player's username?
    }
}

fn send_packet_to_all_clients_event_system(
    mut send_packet_to_all_clients_events: EventReader<SendPacketToAllClientsEvent>,
    mut tcp_stream_component_query: Query<&mut TcpStreamComponent>
) {
    for send_packet_to_all_clients_event in send_packet_to_all_clients_events.iter() {
        for mut tcp_stream_component in tcp_stream_component_query.iter_mut() {
            tcp_stream_component.tcp_stream.write(&bincode::serialize(&send_packet_to_all_clients_event.packet).unwrap()).unwrap();
        }
    }
}

fn listen_for_client_connections_system(
    mut client_connect_events: EventWriter<ClientConnectedEvent>,
    mut commands: Commands,
    tcp_listener_component_query: Query<&TcpListenerComponent>
) {
    if let Ok(tcp_listener_component) = tcp_listener_component_query.get_single() {
        if let Ok((tcp_stream, _)) = tcp_listener_component.tcp_listener.accept() {
            tcp_stream.set_nonblocking(true).unwrap();
            commands.spawn((TcpStreamComponent::new(tcp_stream), Name::new("TcpStream")));
            client_connect_events.send(ClientConnectedEvent);
        }
    }
}

fn listen_for_client_packets_system(
    mut commands: Commands,
    mut client_disconnect_event: EventWriter<ClientDisconnectedEvent>,
    mut recieved_packet_from_client_event: EventWriter<RecievedPacketFromClientEvent>,
    mut tcp_stream_entity_and_tcp_stream_component_query: Query<(Entity, &mut TcpStreamComponent)>
) {
    for (tcp_steam_entity, mut tcp_stream_component) in tcp_stream_entity_and_tcp_stream_component_query.iter_mut() {
        let mut buffer = [0; 1024];

        match tcp_stream_component.tcp_stream.read(&mut buffer) {
            Ok(0) => {
                // send client data with event
                client_disconnect_event.send(ClientDisconnectedEvent);
                tcp_stream_component.tcp_stream.shutdown(Shutdown::Both).unwrap();
                commands.entity(tcp_steam_entity).despawn();
            },
            Ok(packet_length) => recieved_packet_from_client_event.send(RecievedPacketFromClientEvent::new(bincode::deserialize(&buffer[0..packet_length]).unwrap())),
            Err(_) => ()
        }
    }
}
