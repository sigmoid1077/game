// add support for generic packet types
// add support for custom serializers

use bevy::ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    query::With,
    system::{Commands, Query}
};
use crate::{BUFFER_SIZE, Packet, ListenerComponent, StreamComponent};
use std::{
    io::{Read, Write},
    net::{Ipv4Addr, Shutdown, SocketAddr, TcpListener}
};

pub fn read_bind_event_system(
    mut bind_events: EventReader<super::event::write::BindEvent>,
    mut commands: Commands,
    tcp_listener_component_query: Query<&ListenerComponent>
) {
    for bind_event in bind_events.iter() {
        if tcp_listener_component_query.is_empty() {
            let tcp_listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, bind_event.0))).unwrap();
            tcp_listener.set_nonblocking(true).unwrap();
            commands.spawn(ListenerComponent(tcp_listener));
        }
    }
}

pub fn read_unbind_event_system(
    mut commands: Commands,
    entity_with_tcp_listener_component_query: Query<Entity, With<ListenerComponent>>,
    entity_with_tcp_stream_component_query: Query<Entity, With<StreamComponent>>,
    tcp_stream_component_query: Query<&StreamComponent>,
    mut unbind_events: EventReader<super::event::write::UnbindEvent>
) {
    for _unbind_event in unbind_events.iter() {
        for entity_with_tcp_stream_component in entity_with_tcp_stream_component_query.iter() {
            if let Ok(tcp_stream_component) = tcp_stream_component_query.get_component::<StreamComponent>(entity_with_tcp_stream_component) {
                tcp_stream_component.0.shutdown(Shutdown::Both).unwrap();
            }

            commands.entity(entity_with_tcp_stream_component).despawn();
        }
        
        if let Ok(entity_with_tcp_listener_component) = entity_with_tcp_listener_component_query.get_single() {
            commands.entity(entity_with_tcp_listener_component).despawn();
        }
    }
}

pub fn read_send_packet_to_client_event_system<P: Packet>(
    mut send_packet_to_client_events: EventReader<super::event::write::SendPacketToClient<P>>
) {
    for _send_packet_to_client_event in send_packet_to_client_events.iter() {
        todo!();
    }
}

pub fn read_send_packet_to_all_clients_event_system<P: Packet>(
    entity_with_tcp_stream_component_query: Query<Entity, With<StreamComponent>>,
    mut send_packet_to_all_clients_events: EventReader<super::event::write::SendPacketToAllClients<P>>,
    mut tcp_stream_component_query: Query<&mut StreamComponent>
) {
    for send_packet_to_all_clients_event in send_packet_to_all_clients_events.iter() {
        for entity_with_tcp_stream_component in entity_with_tcp_stream_component_query.iter() {
            if let Ok(mut tcp_stream_component) = tcp_stream_component_query.get_component_mut::<StreamComponent>(entity_with_tcp_stream_component) {
                tcp_stream_component.0.write_all(&bincode::serialize(&send_packet_to_all_clients_event.0).unwrap()).unwrap();
            }
        }
    }
}

pub fn write_client_connected_event_system(
    mut client_connected_event: EventWriter<super::event::read::ClientConnectedEvent>,
    mut commands: Commands,
    entity_with_tcp_listener_component_query: Query<Entity, With<ListenerComponent>>,
    tcp_listener_component_query: Query<&ListenerComponent>
) {
    if let Ok(entity_with_tcp_listener_component) = entity_with_tcp_listener_component_query.get_single() {
        if let Ok(tcp_listener_component) = tcp_listener_component_query.get_component::<ListenerComponent>(entity_with_tcp_listener_component) {
            if let Ok((tcp_stream, _)) = tcp_listener_component.0.accept() {
                tcp_stream.set_nonblocking(true).unwrap();
                commands.spawn(StreamComponent(tcp_stream));
                client_connected_event.send(super::event::read::ClientConnectedEvent());
            }
        }
    }
}

pub fn write_client_disconnected_event_and_recieved_packet_from_client_event_system<P: Packet>(
    mut client_disconnected_event: EventWriter<super::event::read::ClientDisconnectedEvent>,
    mut commands: Commands,
    entity_with_tcp_stream_component_query: Query<Entity, With<StreamComponent>>,
    mut recieved_packet_from_client_event: EventWriter<super::event::read::RecievedPacketFromClientEvent<P>>,
    mut tcp_stream_component_query: Query<&mut StreamComponent>
) {
    for entity_with_tcp_stream_component in entity_with_tcp_stream_component_query.iter() {
        if let Ok(mut tcp_stream_component) = tcp_stream_component_query.get_component_mut::<StreamComponent>(entity_with_tcp_stream_component) {
            let mut buffer = [0; BUFFER_SIZE];

            match tcp_stream_component.0.read(&mut buffer) {
                Ok(0) => {
                    client_disconnected_event.send(super::event::read::ClientDisconnectedEvent());
                    tcp_stream_component.0.shutdown(Shutdown::Both).unwrap();
                    commands.entity(entity_with_tcp_stream_component).despawn();
                },
                Ok(packet_length) => recieved_packet_from_client_event.send(super::event::read::RecievedPacketFromClientEvent(bincode::deserialize(&buffer[0..packet_length]).unwrap())),
                _ => ()
            }
        }
    }
}
