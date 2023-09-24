use bevy::{
    app::{App, AppExit, Plugin, Update, Startup},
    DefaultPlugins,
    ecs::event::{EventReader, EventWriter},
    input::{keyboard::KeyboardInput, ButtonState}
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, 
            net::ServerPlugin,
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

fn startup(
    mut bind_event: EventWriter<net::BindEvent>,
) {
    bind_event.send(net::BindEvent::new(7777));
}

fn update(
    mut app_exit_events: EventReader<AppExit>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut unbind_event: EventWriter<net::UnbindEvent>,
    mut recieved_packet_from_client_events: EventReader<net::RecievedPacketFromClientEvent>,
    mut send_packet_to_all_clients_event: EventWriter<net::SendPacketToAllClientsEvent>
) {
    for _ in app_exit_events.iter() {
        unbind_event.send(net::UnbindEvent);
    }

    for recieved_packet_from_client_event in recieved_packet_from_client_events.iter() {
        match &recieved_packet_from_client_event.packet {
            net::Packet::MyPacket(string) => println!("{}", &string)
        }
    }

    for keyboard_input_event in keyboard_input_events.iter() {
        if keyboard_input_event.state == ButtonState::Pressed {
            send_packet_to_all_clients_event.send(net::SendPacketToAllClientsEvent::new(net::Packet::MyPacket(format!("{:?}", keyboard_input_event.key_code.unwrap()))));
        }
    }
}
