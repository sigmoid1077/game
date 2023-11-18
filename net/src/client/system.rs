// add wasm support

use bevy::ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    query::With,
    system::{Commands, Query}
};
use crate::{client::event, BUFFER_SIZE, RecievingPacket, SendingPacket, ServerComponent};
use std::{io::{Read, Write}, net::{TcpStream, Shutdown}};

pub(crate) fn read_connect_event_system(
    mut commands: Commands,
    mut connect_events: EventReader<event::write::ConnectEvent>,
    tcp_stream_component_query: Query<&ServerComponent>
) {
    for connect_event in connect_events.read() {
        if tcp_stream_component_query.iter().count() == 0 {
            let tcp_stream = TcpStream::connect(connect_event.0).unwrap();
            tcp_stream.set_nonblocking(true).unwrap();
            commands.spawn(ServerComponent(tcp_stream));
        }
    }
}

pub(crate) fn read_disconnect_event_system(
    mut commands: Commands,
    mut disconect_events: EventReader<event::write::DisconnectEvent>,
    entity_with_tcp_stream_component_query: Query<Entity, With<ServerComponent>>,
    tcp_stream_component_query: Query<&ServerComponent>
) {
    for _disconnect_event in disconect_events.read() {
        if let Ok(entity_with_tcp_stream_component) = entity_with_tcp_stream_component_query.get_single() {
            if let Ok(tcp_stream_component) = tcp_stream_component_query.get_component::<ServerComponent>(entity_with_tcp_stream_component) {
                tcp_stream_component.0.shutdown(Shutdown::Both).unwrap();
                commands.entity(entity_with_tcp_stream_component).despawn();
            }
        }
    }
}

pub(crate) fn write_send_packet_event_system<Sp: SendingPacket>(
    mut send_packet_events: EventReader<event::write::SendPacketEvent<Sp>>,
    entity_with_tcp_stream_component_query: Query<Entity, With<ServerComponent>>,
    mut tcp_stream_component_query: Query<&mut ServerComponent>
) {
    for send_packet_to_server_event in send_packet_events.read() {
        if let Ok(entity_with_tcp_stream_component) = entity_with_tcp_stream_component_query.get_single() {
            if let Ok(mut tcp_stream_component) = tcp_stream_component_query.get_component_mut::<ServerComponent>(entity_with_tcp_stream_component) {
                tcp_stream_component.0.write_all(send_packet_to_server_event.0.serialize_packet().as_ref()).unwrap();
            }
        }
    }
}

pub(crate) fn write_server_disconnected_event_and_recieved_packet_event_system<Rp: RecievingPacket>(
    mut commands: Commands,
    entity_with_tcp_stream_component_query: Query<Entity, With<ServerComponent>>,
    mut mut_tcp_stream_component_query: Query<&mut ServerComponent>,
    mut server_disconnected_events: EventWriter<event::read::ServerDisconnectedEvent>,
    mut recieved_packet_events: EventWriter<event::read::RecievedPacket<Rp>>
) {
    if let Ok(entity_with_tcp_stream_component) = entity_with_tcp_stream_component_query.get_single() {
        if let Ok(mut tcp_stream_component) = mut_tcp_stream_component_query.get_component_mut::<ServerComponent>(entity_with_tcp_stream_component) {
            let mut buffer = [0; BUFFER_SIZE];
            match tcp_stream_component.0.read(&mut buffer) {
                Ok(0) => {
                    server_disconnected_events.send(event::read::ServerDisconnectedEvent);
                    tcp_stream_component.0.shutdown(Shutdown::Both).unwrap();
                    commands.entity(entity_with_tcp_stream_component).despawn();
                },
                Ok(packet_length) => recieved_packet_events.send(event::read::RecievedPacket(Rp::deserialize_packet(buffer[0..packet_length].as_ref()))),
                _ => ()
            }
        }
    }
}
