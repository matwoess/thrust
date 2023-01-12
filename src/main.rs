mod game_state;
mod input;
mod constant;
mod enemy;
mod ship;
mod shot;
mod goodie;

use std::ops::Add;
use ruscii::app::{App, Config, State};
use ruscii::terminal::{Window, Color};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};
use crate::constant::{BORDER_SIZE, CHAR_HEALTH, CHAR_HEALTH_BAR, CHAR_SHIELD, CHAR_SHIELD_BAR, FPS_LIMIT, GAME_SIZE, HUD_HEALTH_GRANULARITY, HUD_SHIELD_GRANULARITY};

use crate::game_state::GameState;
use crate::input::handle_user_input;

fn main() {
    let mut app = App::config(Config::new().fps(FPS_LIMIT));
    let mut game_state = GameState::new(Vec2::xy(GAME_SIZE.0, GAME_SIZE.1));
    let mut fps_counter = FPSCounter::new();

    app.run(|app_state: &mut State, window: &mut Window| {
        handle_user_input(&mut game_state, app_state);

        let win_size = window.size();
        let mut pencil = Pencil::new(window.canvas_mut());

        if is_game_over(&game_state) {
            render_game_over_screen(&game_state, win_size, &mut pencil);
            return ();
        }

        game_state.update(app_state.step());

        pencil.set_origin((win_size - game_state.dimension) / 2);

        draw_border(&game_state, &mut pencil);
        draw_hud(&game_state, &mut pencil);
        draw_game(&game_state, &mut pencil);
        draw_fps(&mut fps_counter, &mut pencil);
    });
}

fn draw_fps(fps_counter: &mut FPSCounter, pencil: &mut Pencil) {
    fps_counter.update();
    pencil.set_foreground(Color::White);
    pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 0));
}

fn is_game_over(game_state: &GameState) -> bool {
    game_state.health == 0
}

fn render_game_over_screen(game_state: &GameState, win_size: Vec2, pencil: &mut Pencil) {
    let msg = &format!("Game Over  -  score: {}", game_state.score);
    pencil.set_origin(win_size / 2 - Vec2::x(msg.len() / 2));
    pencil.draw_text(msg, Vec2::zero());
}

fn draw_game(game_state: &GameState, pencil: &mut Pencil) {
    game_state.ship.draw(pencil);
    for shot in &game_state.enemy_shots {
        shot.draw(pencil);
    }
    for enemy in &game_state.enemies {
        enemy.draw(pencil);
    }
    for goodie in &game_state.goodies {
        goodie.draw(pencil);
    }
}

fn draw_border(game_state: &GameState, pencil: &mut Pencil) {
    pencil.set_foreground(Color::Grey);
    let border_rect = game_state.dimension.add(Vec2::xy(BORDER_SIZE, BORDER_SIZE));
    pencil.draw_rect(&RectCharset::simple_round_lines(), Vec2::zero(), border_rect);
}

fn draw_hud(game_state: &GameState, pencil: &mut Pencil) {
    let mut pos = Vec2::xy(-2, 0);
    let digits = &format!("{}", game_state.health);
    let chars = digits.chars();
    pos.y -= chars.count() as i32;
    pencil.set_foreground(Color::White);
    for ch in digits.chars() {
        pencil.draw_char(ch, pos);
        pos.y += 1;
    }
    pencil.set_foreground(Color::Red);
    pencil.draw_char(CHAR_HEALTH, pos);
    let curr_health_hud = game_state.health / HUD_HEALTH_GRANULARITY;
    for _ in 0..curr_health_hud {
        pos.y += 1;
        pencil.draw_char(CHAR_HEALTH_BAR, pos);
    }

    let mut pos = Vec2::xy(-4, 0);
    let digits = &format!("{}", game_state.shield);
    let chars = digits.chars();
    pos.y -= chars.count() as i32;
    pencil.set_foreground(Color::White);
    for ch in digits.chars() {
        pencil.draw_char(ch, pos);
        pos.y += 1;
    }
    pencil.set_foreground(Color::Yellow);
    pencil.draw_char(CHAR_SHIELD, pos);
    let curr_shield_hud = game_state.shield / HUD_SHIELD_GRANULARITY;
    for _ in 0..curr_shield_hud {
        pos.y += 1;
        pencil.draw_char(CHAR_SHIELD_BAR, pos);
    }
    pencil.set_foreground(Color::White);
    let status_msg = &format!("Score: {}", game_state.score);
    pencil.draw_text(status_msg, Vec2::xy(25, -1));
}
