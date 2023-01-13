use bevy::prelude::*;
use grasshoppers::MovementPlugin;

fn main() {
    App::new().add_plugins(MinimalPlugins).add_plugin(MovementPlugin).run()
}
