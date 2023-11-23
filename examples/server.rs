use bevy::{
    app::{App, AppExit, Plugin, Startup, Update},
    ecs::event::EventReader,
    input::{keyboard::KeyboardInput, ButtonState},
    DefaultPlugins
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
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
            net::ServerPlugin::<Packet, Packet>(PhantomData, PhantomData),
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

fn startup(mut server: net::ServerSystemParam<Packet, Packet>) {
    let port = std::env::args()
        .collect::<Vec<_>>()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    server.bind(port);
}

fn update(
    mut app_exit_events: EventReader<AppExit>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut server: net::ServerSystemParam<Packet, Packet>
) {
    for _app_exit_event in app_exit_events.read() {
        server.unbind();
    }
    
    for recieved_packet_from_client_event in server.recieved_packets() {
        match &recieved_packet_from_client_event.0 {
            Packet::MyPacket(string) => println!("{}", string.as_str())
        }
    }
    
    for keyboard_input_event in keyboard_input_events.read() {
        if keyboard_input_event.state == ButtonState::Pressed {
            let packet = Packet::MyPacket(String::from("Packet from server"));
            server.send_packet_to_all_clients(packet);
        }
    }
}
