use std::collections::HashMap;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Grasshoppers".to_string(),
                ..default()
            },
            ..default()
        }))
        .init_resource::<KeyCodeToAction>()
        .add_event::<ActionEvent>()
        .add_system(keyboard_input_system)
        .run()
}

#[derive(Debug, Clone, Copy)]
enum MoveDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Move(MoveDirection),
    Jump,
}

struct ActionEvent {
    action: Action,
}

impl ActionEvent {
    pub fn new(action: Action) -> Self {
        Self { action }
    }
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct KeyCodeToAction(HashMap<KeyCode, Action>);

impl FromWorld for KeyCodeToAction {
    fn from_world(_: &mut World) -> Self {
        let mut keycode_to_action_map = HashMap::new();

        keycode_to_action_map.insert(KeyCode::Space, Action::Jump);
        keycode_to_action_map.insert(KeyCode::A, Action::Move(MoveDirection::Left));
        keycode_to_action_map.insert(KeyCode::D, Action::Move(MoveDirection::Right));

        KeyCodeToAction(keycode_to_action_map)
    }
}

fn keyboard_input_system(
    mut action_event_writer: EventWriter<ActionEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    keycode_to_action: Res<KeyCodeToAction>,
) {
    for (keycode, action) in &keycode_to_action.0 {
        if keyboard_input.pressed(*keycode) {
            action_event_writer.send(ActionEvent::new(*action))
        }
    }
}
