mod menu;

use crate::ui::menu::{handle_menu, setup_menu};
use crate::GameState;
use bevy::app::App;
use bevy::prelude::{IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)))
            .add_system(handle_menu.in_set(OnUpdate(GameState::Menu)));
    }
}

#[derive(Default)]
pub struct MenuNavigation(pub u32);
