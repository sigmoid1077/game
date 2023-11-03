use bevy::{
    app::{App, AppExit, Plugin, Startup, Update},
    ecs::event::{EventReader, EventWriter},
    input::{keyboard::KeyboardInput, ButtonState},
    DefaultPlugins
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use serde::{Serialize, Deserialize};
use std::net::ToSocketAddrs;
use util::net;

#[derive(Serialize, Deserialize)]
enum Packet {
    MyPacket(String),
}

fn main() {
    App::new()
        .add_plugins((
            net::ClientPlugin::<Packet>,
            DefaultPlugins,
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
    mut recieved_packet_events: EventReader<net::client::event::read::RecievedPacket>,
    mut send_packet_event: EventWriter<net::client::event::write::SendPacketEvent>
) {
    for _ in app_exit_events.iter() {
        disconnect_event.send(net::client::event::write::DisconnectEvent);
    }

    for recieved_packet_from_server_event in recieved_packet_events.iter() {
        match &recieved_packet_from_server_event.0 {
            net::Packet::MyPacket(string) => println!("{}", &string)
        }
    }

    for keyboard_input_event in keyboard_input_events.iter() {
        if keyboard_input_event.state == ButtonState::Pressed {
            send_packet_event.send(net::client::event::write::SendPacketEvent(net::Packet::MyPacket(format!("{:?}", keyboard_input_event.key_code.unwrap()))));
        }
    }
}
