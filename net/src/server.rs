use bevy::{ecs::system::SystemParam, prelude::*};
use crate::{ClientComponent, Packet, ServerComponent};
use std::{io::{Read, Write}, marker::PhantomData, net::{Ipv4Addr, Shutdown, SocketAddr, TcpListener}};

pub struct ServerPlugin<SendPacket: Packet, RecvPacket: Packet>(PhantomData<SendPacket>, PhantomData<RecvPacket>);

impl<SendPacket: Packet, RecvPacket: Packet> Plugin for ServerPlugin<SendPacket, RecvPacket> {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BindEvent>()
            .add_event::<UnbindEvent>()
            .add_event::<SendToEvent<SendPacket>>()
            .add_event::<SendEvent<SendPacket>>()
            .add_event::<ConnectedEvent>()
            .add_event::<DisconnectedEvent>()
            .add_event::<RecvEvent<RecvPacket>>()
            .add_systems(Update, (
                read_bind_event,
                read_unbind_event,
                read_send_to_event::<SendPacket>,
                read_send_event::<SendPacket>,
                write_connected_event,
                write_disconnected_and_recv_events::<RecvPacket>
            ));
    }
}

#[derive(SystemParam)]
pub struct ServerSystemParam<'w, 's, SendPacket: Packet, RecvPacket: Packet> {
    bind_event: EventWriter<'w, BindEvent>,
    unbind_event: EventWriter<'w, UnbindEvent>,
    send_packet_to_all_clients_event: EventWriter<'w, SendEvent<SendPacket>>,
    recieved_packet_from_client_events: EventReader<'w, 's, RecvEvent<RecvPacket>>
}

impl<'w, 's, SendPacket: Packet, RecvPacket: Packet> ServerSystemParam<'w, 's, SendPacket, RecvPacket> {
    pub fn bind(&mut self, port: u16) {
        self.bind_event.send(BindEvent(port));
    }

    pub fn unbind(&mut self) {
        self.unbind_event.send(UnbindEvent);
    }

    pub fn send_packet_to_client(&mut self) {
        unimplemented!();
    }

    pub fn send_packet_to_all_clients(&mut self, packet: SendPacket) {
        self.send_packet_to_all_clients_event.send(SendEvent(packet));
    }

    pub fn recieved_packets(&mut self) -> impl Iterator<Item=&RecvPacket> {
        self.recieved_packet_from_client_events.read().map(|recieved_packet_event| &recieved_packet_event.0)
    }
}

#[derive(Event)]
pub struct BindEvent(pub u16);

#[derive(Event)]
pub struct UnbindEvent;

#[derive(Event)]
pub struct SendToEvent<SendPacket: Packet>(/* client */ pub SendPacket);

#[derive(Event)]
pub struct SendEvent<SendPacket: Packet>(pub SendPacket);

#[derive(Event)]
pub struct ConnectedEvent(/* client */);

#[derive(Event)]
pub struct DisconnectedEvent(/* client */);

#[derive(Event)]
pub struct RecvEvent<RecvPacket: Packet>(/* client */ pub RecvPacket);

pub(crate) fn read_bind_event(
    mut bind_events: EventReader<BindEvent>,
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

pub(crate) fn read_unbind_event(
    mut commands: Commands,
    entity_with_server_component_query: Query<Entity, With<ServerComponent>>,
    entity_with_client_component_query: Query<Entity, With<ClientComponent>>,
    client_component_query: Query<&ClientComponent>,
    mut unbind_events: EventReader<UnbindEvent>
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

pub(crate) fn read_send_to_event<Sp: Packet>(
    mut send_packet_to_client_events: EventReader<SendToEvent<Sp>>
) {
    for _send_packet_to_client_event in send_packet_to_client_events.read() {
        unimplemented!();
    }
}

pub(crate) fn read_send_event<Sp: Packet>(
    entity_with_client_component_query: Query<Entity, With<ClientComponent>>,
    mut send_packet_to_all_clients_events: EventReader<SendEvent<Sp>>,
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

pub(crate) fn write_connected_event(
    mut client_connected_event: EventWriter<ConnectedEvent>,
    mut commands: Commands,
    entity_with_server_component_query: Query<Entity, With<ServerComponent>>,
    client_component_query: Query<&ServerComponent>
) {
    if let Ok(entity_with_server_component) = entity_with_server_component_query.get_single() {
        if let Ok(client_component) = client_component_query.get_component::<ServerComponent>(entity_with_server_component) {
            if let Ok((tcp_stream, _)) = client_component.0.accept() {
                tcp_stream.set_nonblocking(true).unwrap();
                commands.spawn(ClientComponent(tcp_stream));
                client_connected_event.send(ConnectedEvent());
            }
        }
    }
}

pub(crate) fn write_disconnected_and_recv_events<Rp: Packet>(
    mut client_disconnected_event: EventWriter<DisconnectedEvent>,
    mut commands: Commands,
    entity_with_client_component_query: Query<Entity, With<ClientComponent>>,
    mut recieved_packet_from_client_event: EventWriter<RecvEvent<Rp>>,
    mut client_component_query: Query<&mut ClientComponent>
) {
    for entity_with_client_component in entity_with_client_component_query.iter() {
        if let Ok(mut client_component) = client_component_query.get_component_mut::<ClientComponent>(entity_with_client_component) {
            let mut buf = [0; 1024];

            match client_component.0.read(&mut buf) {
                Ok(0) => {
                    client_disconnected_event.send(DisconnectedEvent());
                    client_component.0.shutdown(Shutdown::Both).unwrap();
                    commands.entity(entity_with_client_component).despawn();
                },
                Ok(packet_length) => recieved_packet_from_client_event.send(RecvEvent(Rp::deserialize_packet(&buf[0..packet_length]))),
                _ => ()
            }
        }
    }
}
