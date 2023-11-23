// add wasm support

use bevy::ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    query::With,
    system::{Commands, Query, Res, ResMut}
};
use crate::{BUFFER_SIZE, client::{event}, Packet, ClientComponent};
use std::{io::{Read, Write}, net::{TcpStream, Shutdown}};

pub(crate) fn read_connect_event_system(
    mut commands: Commands,
    mut connect_events: EventReader<event::write::ConnectEvent>,
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

pub(crate) fn read_disconnect_event_system(
    mut commands: Commands,
    mut disconect_events: EventReader<event::write::DisconnectEvent>,
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

pub(crate) fn write_send_packet_event_system<Sp: Packet>(
    mut send_packet_events: EventReader<event::write::SendPacketEvent<Sp>>,
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

pub(crate) fn write_server_disconnected_event_and_recieved_packet_event_system<Rp: Packet>(
    mut commands: Commands,
    entity_with_client_component_query: Query<Entity, With<ClientComponent>>,
    mut mut_client_component_query: Query<&mut ClientComponent>,
    mut server_disconnected_events: EventWriter<event::read::ServerDisconnectedEvent>,
    mut recieved_packet_events: EventWriter<event::read::RecievedPacket<Rp>>
) {
    if let Ok(entity_with_client_component) = entity_with_client_component_query.get_single() {
        if let Ok(mut client_component) = mut_client_component_query.get_component_mut::<ClientComponent>(entity_with_client_component) {
            let mut buffer = [0; BUFFER_SIZE];
            match client_component.0.read(&mut buffer) {
                Ok(0) => {
                    server_disconnected_events.send(event::read::ServerDisconnectedEvent);
                    client_component.0.shutdown(Shutdown::Both).unwrap();
                    commands.entity(entity_with_client_component).despawn();
                },
                Ok(packet_length) => recieved_packet_events.send(event::read::RecievedPacket(Rp::deserialize_packet(&buffer[0..packet_length]))),
                _ => ()
            }
        }
    }
}
