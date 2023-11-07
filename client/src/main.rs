use bevy::{
    app::{App, AppExit, Plugin, Startup, Update},
    ecs::event::{EventReader, EventWriter},
    input::{keyboard::KeyboardInput, ButtonState},
    DefaultPlugins
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use serde::{Serialize, Deserialize};
use std::{net::ToSocketAddrs, marker::PhantomData};
use util::net;

#[derive(Serialize, Deserialize)]
enum Packet {
    MyPacket(String),
}

impl net::SendingPacket for Packet {
    fn serialize_packet(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

impl net::RecievingPacket for Packet {
    fn deserialize_packet(buffer: &[u8]) -> Self {
        bincode::deserialize(buffer).unwrap()
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            net::ClientPlugin::<Packet, Packet>(PhantomData, PhantomData),
            TestPlugin,
            WorldInspectorPlugin::new()
        ))
        .run();
}

struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, update);
    }
}

fn startup(mut connect_event: EventWriter<net::client::event::write::ConnectEvent>) {
    connect_event.send(net::client::event::write::ConnectEvent(std::env::args().collect::<Vec<_>>().last().unwrap().to_socket_addrs().unwrap().collect::<Vec<_>>().first().unwrap().to_owned()));
}

fn update(
    mut app_exit_events: EventReader<AppExit>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut disconnect_event: EventWriter<net::client::event::write::DisconnectEvent>,
    mut recieved_packet_events: EventReader<net::client::event::read::RecievedPacket<Packet>>,
    mut send_packet_event: EventWriter<net::client::event::write::SendPacketEvent<Packet>>
) {
    for _ in app_exit_events.read() {
        disconnect_event.send(net::client::event::write::DisconnectEvent);
    }

    for recieved_packet_from_server_event in recieved_packet_events.read() {
        match &recieved_packet_from_server_event.0 {
            Packet::MyPacket(string) => println!("{}", &string)
        }
    }

    for keyboard_input_event in keyboard_input_events.read() {
        if keyboard_input_event.state == ButtonState::Pressed {
            send_packet_event.send(net::client::event::write::SendPacketEvent(Packet::MyPacket(format!("{:?}", keyboard_input_event.key_code.unwrap()))));
        }
    }
}
