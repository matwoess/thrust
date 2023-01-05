use ruscii::app::State;
use ruscii::keyboard::{Key, KeyEvent};
use crate::constant::{MOVE_SPEED_X, MOVE_SPEED_Y};
use crate::game_state::GameState;

pub fn handle_user_input(mut game_state: &mut GameState, app_state: &mut State) {
    for key_event in app_state.keyboard().last_key_events() {
        match key_event {
            KeyEvent::Pressed(Key::Esc) | KeyEvent::Pressed(Key::Q) => app_state.stop(),
            KeyEvent::Pressed(Key::Space) => game_state.ship.auto_shoot = !game_state.ship.auto_shoot,
            _ => (),
        }
    }

    for key_down in app_state.keyboard().get_keys_down() {
        match key_down {
            Key::W => game_state.move_ship_y(-MOVE_SPEED_Y),
            Key::S => game_state.move_ship_y(MOVE_SPEED_Y),
            Key::A => game_state.move_ship_x(-MOVE_SPEED_X),
            Key::D => game_state.move_ship_x(MOVE_SPEED_X),
            _ => (),
        }
    }
}