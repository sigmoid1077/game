use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};
use std::net::{TcpStream, TcpListener};

const BUFFER_SIZE: usize = 1024;

#[derive(Component)]
struct TcpStreamComponent(pub TcpStream);

#[derive(Component)]
struct TcpListenerComponent(pub TcpListener);

#[derive(Serialize, Deserialize)]
pub enum Packet {
    MyPacket(String),
}

pub mod client {
    use bevy::{
        app::{App, Plugin, Update}, 
        ecs::{
            entity::Entity,
            event::{EventReader, EventWriter},
            query::With,
            system::{Commands, Query}
        }
    };
    use crate::net::{BUFFER_SIZE, TcpStreamComponent};
    use std::{
        io::{Read, Write}, 
        net::{TcpStream, Shutdown}
    };

    pub struct ClientPlugin;

    impl Plugin for ClientPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_event::<event::write::ConnectEvent>()
                .add_event::<event::write::DisconnectEvent>()
                .add_event::<event::write::SendPacketEvent>()
                .add_event::<event::read::ServerDisconnectedEvent>()
                .add_event::<event::read::RecievedPacket>()
                .add_systems(Update, (
                    read_connect_event_system,
                    read_disconnect_event_system,
                    write_send_packet_event_system,
                    write_server_disconnected_event_and_recieved_packet_event_system
                ));
        }
    }

    pub mod event {
        pub mod write {
            use bevy::ecs::event::Event;
            use crate::net::Packet;
            use std::net::SocketAddr;

            // add field for server ip
            #[derive(Event)]
            pub struct ConnectEvent(pub SocketAddr);

            #[derive(Event)]
            pub struct DisconnectEvent;

            #[derive(Event)]
            pub struct SendPacketEvent(pub Packet);
        }

        pub mod read {
            use bevy::ecs::event::Event;
            use crate::net::Packet;

            #[derive(Event)]
            pub struct ServerDisconnectedEvent;

            #[derive(Event)]
            pub struct RecievedPacket(pub Packet);
        }
    }

    fn read_connect_event_system(
        mut commands: Commands,
        mut connect_events: EventReader<event::write::ConnectEvent>,
        tcp_stream_component_query: Query<&TcpStreamComponent>
    ) {
        for connect_event in connect_events.iter() {
            if tcp_stream_component_query.iter().count() == 0 {
                let tcp_stream = TcpStream::connect(connect_event.0).unwrap();
                tcp_stream.set_nonblocking(true).unwrap();
                commands.spawn(TcpStreamComponent(tcp_stream));
            }
        }
    }

    fn read_disconnect_event_system(
        mut commands: Commands,
        mut disconect_events: EventReader<event::write::DisconnectEvent>,
        entity_with_tcp_stream_component_query: Query<Entity, With<TcpStreamComponent>>,
        tcp_stream_component_query: Query<&TcpStreamComponent>
    ) {
        for _disconnect_event in disconect_events.iter() {
            if let Ok(entity_with_tcp_stream_component) = entity_with_tcp_stream_component_query.get_single() {
                if let Ok(tcp_stream_component) = tcp_stream_component_query.get_component::<TcpStreamComponent>(entity_with_tcp_stream_component) {
                    tcp_stream_component.0.shutdown(Shutdown::Both).unwrap();
                    commands.entity(entity_with_tcp_stream_component).despawn();
                }
            }
        }
    }

    fn write_send_packet_event_system(
        mut send_packet_events: EventReader<event::write::SendPacketEvent>,
        entity_with_tcp_stream_component_query: Query<Entity, With<TcpStreamComponent>>,
        mut tcp_stream_component_query: Query<&mut TcpStreamComponent>
    ) {
        for send_packet_to_server_event in send_packet_events.iter() {
            if let Ok(entity_with_tcp_stream_component) = entity_with_tcp_stream_component_query.get_single() {
                if let Ok(mut tcp_stream_component) = tcp_stream_component_query.get_component_mut::<TcpStreamComponent>(entity_with_tcp_stream_component) {
                    tcp_stream_component.0.write_all(&bincode::serialize(&send_packet_to_server_event.0).unwrap()).unwrap();
                }
            }
        }
    }

    fn write_server_disconnected_event_and_recieved_packet_event_system(
        mut commands: Commands,
        entity_with_tcp_stream_component_query: Query<Entity, With<TcpStreamComponent>>,
        mut mut_tcp_stream_component_query: Query<&mut TcpStreamComponent>,
        mut server_disconnected_events: EventWriter<event::read::ServerDisconnectedEvent>,
        mut recieved_packet_events: EventWriter<event::read::RecievedPacket>
    ) {
        if let Ok(entity_with_tcp_stream_component) = entity_with_tcp_stream_component_query.get_single() {
            if let Ok(mut tcp_stream_component) = mut_tcp_stream_component_query.get_component_mut::<TcpStreamComponent>(entity_with_tcp_stream_component) {
                let mut buffer = [0; BUFFER_SIZE];
                match tcp_stream_component.0.read(&mut buffer) {
                    Ok(0) => {
                        server_disconnected_events.send(event::read::ServerDisconnectedEvent);
                        tcp_stream_component.0.shutdown(Shutdown::Both).unwrap();
                        commands.entity(entity_with_tcp_stream_component).despawn();
                    },
                    Ok(packet_length) => {
                        recieved_packet_events.send(event::read::RecievedPacket(bincode::deserialize(&buffer[0..packet_length]).unwrap()))
                    },
                    _ => ()
                }
            }
        }
    }
}

pub mod server {
    use bevy::{
        app::{App, Plugin, Update}, 
        ecs::{
            entity::Entity,
            event::{EventReader, EventWriter},
            query::With,
            system::{Commands, Query}
        }
    };
    use crate::net::{BUFFER_SIZE, TcpListenerComponent, TcpStreamComponent};
    use std::{
        io::{Read, Write}, 
        net::{TcpListener, Shutdown}
    };
    
    pub struct ServerPlugin;

    impl Plugin for ServerPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_event::<event::write::BindEvent>()
                .add_event::<event::write::UnbindEvent>()
                .add_event::<event::write::SendPacketToClient>()
                .add_event::<event::write::SendPacketToAllClients>()
                .add_event::<event::read::ClientConnectedEvent>()
                .add_event::<event::read::ClientDisconnectedEvent>()
                .add_event::<event::read::RecievedPacketFromClientEvent>()
                .add_systems(Update, (
                    read_bind_event_system,
                    read_unbind_event_system,
                    read_send_packet_to_client_event_system,
                    read_send_packet_to_all_clients_event_system,
                    write_client_connected_event_system,
                    write_client_disconnected_event_and_recieved_packet_from_client_event_system
                ));
        }
    }

    pub mod event {
        pub mod write {
            use bevy::ecs::event::Event;
            use crate::net::Packet;

            #[derive(Event)]
            pub struct BindEvent(pub u16);

            #[derive(Event)]
            pub struct UnbindEvent;

            #[derive(Event)]
            pub struct SendPacketToClient(/* client */ pub Packet);

            #[derive(Event)]
            pub struct SendPacketToAllClients(pub Packet);
        }

        pub mod read {
            use bevy::ecs::event::Event;
            use crate::net::Packet;

            #[derive(Event)]
            pub struct ClientConnectedEvent(/* client */);

            #[derive(Event)]
            pub struct ClientDisconnectedEvent(/* client */);

            #[derive(Event)]
            pub struct RecievedPacketFromClientEvent(/* client */ pub Packet);
        }
    }

    fn read_bind_event_system(
        mut bind_events: EventReader<event::write::BindEvent>,
        mut commands: Commands,
        tcp_component_query: Query<&TcpListenerComponent>
    ) {
        for _bind_event in bind_events.iter() {
            if tcp_component_query.is_empty() {
                let tcp_listener = TcpListener::bind("127.0.0.1:2560").unwrap();
                tcp_listener.set_nonblocking(true).unwrap();
                commands.spawn(TcpListenerComponent(tcp_listener));
            }
        }
    }

    fn read_unbind_event_system(
        mut commands: Commands,
        entity_with_tcp_listener_component_query: Query<Entity, With<TcpListenerComponent>>,
        entity_with_tcp_stream_component_query: Query<Entity, With<TcpStreamComponent>>,
        tcp_stream_component_query: Query<&TcpStreamComponent>,
        mut unbind_events: EventReader<event::write::UnbindEvent>
    ) {
        for _unbind_event in unbind_events.iter() {
            for entity_with_tcp_stream_component in entity_with_tcp_stream_component_query.iter() {
                if let Ok(tcp_stream_component) = tcp_stream_component_query.get_component::<TcpStreamComponent>(entity_with_tcp_stream_component) {
                    tcp_stream_component.0.shutdown(Shutdown::Both).unwrap();
                }

                commands.entity(entity_with_tcp_stream_component).despawn();
            }
            
            if let Ok(entity_with_tcp_listener_component) = entity_with_tcp_listener_component_query.get_single() {
                commands.entity(entity_with_tcp_listener_component).despawn();
            }
        }
    }

    fn read_send_packet_to_client_event_system(
        mut send_packet_to_client_events: EventReader<event::write::SendPacketToClient>
    ) {
        for _send_packet_to_client_event in send_packet_to_client_events.iter() {
            todo!();
        }
    }
    
    fn read_send_packet_to_all_clients_event_system(
        entity_with_tcp_stream_component_query: Query<Entity, With<TcpStreamComponent>>,
        mut send_packet_to_all_clients_events: EventReader<event::write::SendPacketToAllClients>,
        mut tcp_stream_component_query: Query<&mut TcpStreamComponent>
    ) {
        for send_packet_to_all_clients_event in send_packet_to_all_clients_events.iter() {
            for entity_with_tcp_stream_component in entity_with_tcp_stream_component_query.iter() {
                if let Ok(mut tcp_stream_component) = tcp_stream_component_query.get_component_mut::<TcpStreamComponent>(entity_with_tcp_stream_component) {
                    tcp_stream_component.0.write_all(&bincode::serialize(&send_packet_to_all_clients_event.0).unwrap()).unwrap();
                }
            }
        }
    }
    
    fn write_client_connected_event_system(
        mut client_connected_event: EventWriter<event::read::ClientConnectedEvent>,
        mut commands: Commands,
        entity_with_tcp_listener_component_query: Query<Entity, With<TcpListenerComponent>>,
        tcp_listener_component_query: Query<&TcpListenerComponent>
    ) {
        if let Ok(entity_with_tcp_listener_component) = entity_with_tcp_listener_component_query.get_single() {
            if let Ok(tcp_listener_component) = tcp_listener_component_query.get_component::<TcpListenerComponent>(entity_with_tcp_listener_component) {
                if let Ok((tcp_stream, _)) = tcp_listener_component.0.accept() {
                    tcp_stream.set_nonblocking(true).unwrap();
                    commands.spawn(TcpStreamComponent(tcp_stream));
                    client_connected_event.send(event::read::ClientConnectedEvent());
                }
            }
        }
    }
    
    fn write_client_disconnected_event_and_recieved_packet_from_client_event_system(
        mut client_disconnected_event: EventWriter<event::read::ClientDisconnectedEvent>,
        mut commands: Commands,
        entity_with_tcp_stream_component_query: Query<Entity, With<TcpStreamComponent>>,
        mut recieved_packet_from_client_event: EventWriter<event::read::RecievedPacketFromClientEvent>,
        mut tcp_stream_component_query: Query<&mut TcpStreamComponent>
    ) {
        for entity_with_tcp_stream_component in entity_with_tcp_stream_component_query.iter() {
            if let Ok(mut tcp_stream_component) = tcp_stream_component_query.get_component_mut::<TcpStreamComponent>(entity_with_tcp_stream_component) {
                let mut buffer = [0; BUFFER_SIZE];

                match tcp_stream_component.0.read(&mut buffer) {
                    Ok(0) => {
                        client_disconnected_event.send(event::read::ClientDisconnectedEvent());
                        tcp_stream_component.0.shutdown(Shutdown::Both).unwrap();
                        commands.entity(entity_with_tcp_stream_component).despawn();
                    },
                    Ok(packet_length) => {
                        recieved_packet_from_client_event.send(event::read::RecievedPacketFromClientEvent(bincode::deserialize(&buffer[0..packet_length]).unwrap()))
                    },
                    _ => ()
                }
            }
        }
    }
}
