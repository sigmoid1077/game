use bevy::prelude::*;
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
        app.add_systems(Update, (send, recieve));
    }
}

fn send(
    input_keycode: Res<Input<KeyCode>>,
    mut bind_event: EventWriter<net::BindEvent>,
    mut unbind_event: EventWriter<net::UnbindEvent>,
    mut send_packet_to_all_clients_event: EventWriter<net::SendPacketToAllClientsEvent>
) {
    if input_keycode.just_pressed(KeyCode::Q) {
        bind_event.send(net::BindEvent::new(7777));
    }

    if input_keycode.just_pressed(KeyCode::W) {
        unbind_event.send(net::UnbindEvent);
    }

    if input_keycode.just_pressed(KeyCode::E) {
        send_packet_to_all_clients_event.send(net::SendPacketToAllClientsEvent);
    }
}

fn recieve(
    mut client_connected_events: EventReader<net::ClientConnectedEvent>,
    mut client_disconnected_events: EventReader<net::ClientDisconnectedEvent>,
    mut recieved_packet_from_client_events: EventReader<net::RecievedPacketFromClientEvent>
) {
    for _ in client_connected_events.iter() {
        println!("Client connected.")
    }

    for _ in client_disconnected_events.iter() {
        println!("Client disconnected.")
    }

    for recieved_packet_from_client_event in recieved_packet_from_client_events.iter() {
        match &recieved_packet_from_client_event.packet {
            net::Packet::MyPacket(string) => println!("{}", &string)
        }
    }
}
