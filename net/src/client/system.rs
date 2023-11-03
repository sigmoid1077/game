// add support for generic packet types
// add support for custom serializers

use bevy::ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    query::With,
    system::{Commands, Query}
};
use crate::{BUFFER_SIZE, Packet, StreamComponent};
use std::{
    io::{Read, Write},
    net::{TcpStream, Shutdown}
};

#[cfg(not(target_arch = "wasm32"))]
pub fn read_connect_event_system(
    mut commands: Commands,
    mut connect_events: EventReader<super::event::write::ConnectEvent>,
    tcp_stream_component_query: Query<&StreamComponent>
) {
    for connect_event in connect_events.iter() {
        if tcp_stream_component_query.iter().count() == 0 {
            let tcp_stream = TcpStream::connect(connect_event.0).unwrap();
            tcp_stream.set_nonblocking(true).unwrap();
            commands.spawn(StreamComponent(tcp_stream));
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn read_connect_event_system() { }

#[cfg(not(target_arch = "wasm32"))]
pub fn read_disconnect_event_system(
    mut commands: Commands,
    mut disconect_events: EventReader<super::event::write::DisconnectEvent>,
    entity_with_tcp_stream_component_query: Query<Entity, With<StreamComponent>>,
    tcp_stream_component_query: Query<&StreamComponent>
) {
    for _disconnect_event in disconect_events.iter() {
        if let Ok(entity_with_tcp_stream_component) = entity_with_tcp_stream_component_query.get_single() {
            if let Ok(tcp_stream_component) = tcp_stream_component_query.get_component::<StreamComponent>(entity_with_tcp_stream_component) {
                tcp_stream_component.0.shutdown(Shutdown::Both).unwrap();
                commands.entity(entity_with_tcp_stream_component).despawn();
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn read_disconnect_event_system() { }

#[cfg(not(target_arch = "wasm32"))]
pub fn write_send_packet_event_system<P: Packet>(
    mut send_packet_events: EventReader<super::event::write::SendPacketEvent<P>>,
    entity_with_tcp_stream_component_query: Query<Entity, With<StreamComponent>>,
    mut tcp_stream_component_query: Query<&mut StreamComponent>
) {
    for send_packet_to_server_event in send_packet_events.iter() {
        if let Ok(entity_with_tcp_stream_component) = entity_with_tcp_stream_component_query.get_single() {
            if let Ok(mut tcp_stream_component) = tcp_stream_component_query.get_component_mut::<StreamComponent>(entity_with_tcp_stream_component) {
                tcp_stream_component.0.write_all(&bincode::serialize(&send_packet_to_server_event.0).unwrap()).unwrap();
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn write_send_packet_event_system<P: Packet>() { }

#[cfg(not(target_arch = "wasm32"))]
pub fn write_server_disconnected_event_and_recieved_packet_event_system<'de, P: Packet>(
    mut commands: Commands,
    entity_with_tcp_stream_component_query: Query<Entity, With<StreamComponent>>,
    mut mut_tcp_stream_component_query: Query<&mut StreamComponent>,
    mut server_disconnected_events: EventWriter<super::event::read::ServerDisconnectedEvent>,
    mut recieved_packet_events: EventWriter<super::event::read::RecievedPacket<P>>
) {
    if let Ok(entity_with_tcp_stream_component) = entity_with_tcp_stream_component_query.get_single() {
        if let Ok(mut tcp_stream_component) = mut_tcp_stream_component_query.get_component_mut::<StreamComponent>(entity_with_tcp_stream_component) {
            let mut buffer = [0; BUFFER_SIZE];
            match tcp_stream_component.0.read(&mut buffer) {
                Ok(0) => {
                    server_disconnected_events.send(super::event::read::ServerDisconnectedEvent);
                    tcp_stream_component.0.shutdown(Shutdown::Both).unwrap();
                    commands.entity(entity_with_tcp_stream_component).despawn();
                },
                Ok(packet_length) => recieved_packet_events.send(super::event::read::RecievedPacket(bincode::deserialize(buffer[0..packet_length].as_ref()).unwrap())),
                _ => ()
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn write_server_disconnected_event_and_recieved_packet_event_system<'de, P: Packet>() { }
