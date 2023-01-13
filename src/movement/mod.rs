use bevy::prelude::{Plugin, Component, Query, Commands, Entity, With};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
       
    }
}

pub enum Direction {
	Left,
	Right
}

#[derive(Component)]
pub struct Move {
	direction: Direction
}

fn move_system(mut commands: Commands, query: Query<(Entity, &Move)>) {

}