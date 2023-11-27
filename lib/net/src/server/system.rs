// add wasm support

use bevy::ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    query::With,
    system::{Commands, Query}
};
use crate::{ServerComponent, Packet, server::event, ClientComponent};
use std::{
    io::{Read, Write}, 
    net::{Ipv4Addr, Shutdown, SocketAddr, TcpListener}
};

pub(crate) fn read_bind_event_system(
    mut bind_events: EventReader<event::write::BindEvent>,
    mut commands: Commands,
    server_component_query: Query<&ServerComponent>
) {
    for bind_event in bind_events.read() {
        if server_component_query.is_empty() {
            let tcp_listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, bind_event.0))).unwrap();
            tcp_listener.set_nonblocking(true).unwrap();
            commands.spawn(ServerComponent(tcp_listener));
        }
    }
}

pub(crate) fn read_unbind_event_system(
    mut commands: Commands,
    entity_with_server_component_query: Query<Entity, With<ServerComponent>>,
    entity_with_client_component_query: Query<Entity, With<ClientComponent>>,
    client_component_query: Query<&ClientComponent>,
    mut unbind_events: EventReader<event::write::UnbindEvent>
) {
    for _unbind_event in unbind_events.read() {
        for entity_with_client_component in entity_with_client_component_query.iter() {
            if let Ok(client_component) = client_component_query.get_component::<ClientComponent>(entity_with_client_component) {
                client_component.0.shutdown(Shutdown::Both).unwrap();
            }

            commands.entity(entity_with_client_component).despawn();
        }
        
        if let Ok(entity_with_server_component) = entity_with_server_component_query.get_single() {
            commands.entity(entity_with_server_component).despawn();
        }
    }
}

pub(crate) fn read_send_packet_to_client_event_system<Sp: Packet>(
    mut send_packet_to_client_events: EventReader<event::write::SendPacketToClient<Sp>>
) {
    for _send_packet_to_client_event in send_packet_to_client_events.read() {
        unimplemented!();
    }
}

pub(crate) fn read_send_packet_to_all_clients_event_system<Sp: Packet>(
    entity_with_client_component_query: Query<Entity, With<ClientComponent>>,
    mut send_packet_to_all_clients_events: EventReader<event::write::SendPacketToAllClients<Sp>>,
    mut client_component_query: Query<&mut ClientComponent>
) {
    for send_packet_to_all_clients_event in send_packet_to_all_clients_events.read() {
        for entity_with_client_component in entity_with_client_component_query.iter() {
            if let Ok(mut client_component) = client_component_query.get_component_mut::<ClientComponent>(entity_with_client_component) {
                client_component.0.write_all(&send_packet_to_all_clients_event.0.serialize_packet()).unwrap();
            }
        }
    }
}

pub(crate) fn write_client_connected_event_system(
    mut client_connected_event: EventWriter<event::read::ClientConnectedEvent>,
    mut commands: Commands,
    entity_with_server_component_query: Query<Entity, With<ServerComponent>>,
    client_component_query: Query<&ServerComponent>
) {
    if let Ok(entity_with_server_component) = entity_with_server_component_query.get_single() {
        if let Ok(client_component) = client_component_query.get_component::<ServerComponent>(entity_with_server_component) {
            if let Ok((tcp_stream, _)) = client_component.0.accept() {
                tcp_stream.set_nonblocking(true).unwrap();
                commands.spawn(ClientComponent(tcp_stream));
                client_connected_event.send(event::read::ClientConnectedEvent());
            }
        }
    }
}

pub(crate) fn write_client_disconnected_event_and_recieved_packet_from_client_event_system<const BUFFER_SIZE: usize, Rp: Packet>(
    mut client_disconnected_event: EventWriter<event::read::ClientDisconnectedEvent>,
    mut commands: Commands,
    entity_with_client_component_query: Query<Entity, With<ClientComponent>>,
    mut recieved_packet_from_client_event: EventWriter<event::read::RecievedPacketFromClientEvent<Rp>>,
    mut client_component_query: Query<&mut ClientComponent>
) {
    for entity_with_client_component in entity_with_client_component_query.iter() {
        if let Ok(mut client_component) = client_component_query.get_component_mut::<ClientComponent>(entity_with_client_component) {
            let mut buffer = [0; BUFFER_SIZE];

            match client_component.0.read(&mut buffer) {
                Ok(0) => {
                    client_disconnected_event.send(event::read::ClientDisconnectedEvent());
                    client_component.0.shutdown(Shutdown::Both).unwrap();
                    commands.entity(entity_with_client_component).despawn();
                },
                Ok(packet_length) => recieved_packet_from_client_event.send(event::read::RecievedPacketFromClientEvent(Rp::deserialize_packet(&buffer[0..packet_length]))),
                _ => ()
            }
        }
    }
}
