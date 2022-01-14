use bevy::prelude::*;

use crate::GameState;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub reroll: bool,
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if GameControl::Reroll.just_released(&keyboard_input) {
        actions.reroll = true;
    } else {
        actions.reroll = false;
    }
}

enum GameControl {
    Reroll,
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Reroll => keyboard_input.just_released(KeyCode::Space),
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Reroll => keyboard_input.pressed(KeyCode::Space),
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Reroll => keyboard_input.just_pressed(KeyCode::Space),
        }
    }
}
