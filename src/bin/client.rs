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
        .run()
}
