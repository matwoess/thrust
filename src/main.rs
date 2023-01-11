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
use crate::constant::{BORDER_SIZE, FPS_LIMIT, GAME_SIZE, HUD_HEALTH_GRANULARITY};

use crate::game_state::GameState;
use crate::input::handle_user_input;

fn main() {
    let mut app = App::config(Config::new().fps(FPS_LIMIT));
    let mut game_state = GameState::new(Vec2::xy(GAME_SIZE.0, GAME_SIZE.1));
    let mut fps_counter = FPSCounter::new();

    app.run(|app_state: &mut State, window: &mut Window| {
        handle_user_input(&mut game_state, app_state);
        game_state.update(app_state.step());

        let win_size = window.size();
        let mut pencil = Pencil::new(window.canvas_mut());

        draw_fps(&mut fps_counter, &mut pencil);

        if is_game_over(&mut game_state) {
            render_game_over_screen(&game_state, win_size, &mut pencil);
            return ();
        }
        draw_game(&mut game_state, win_size, pencil)
    });
}

fn draw_fps(fps_counter: &mut FPSCounter, pencil: &mut Pencil) {
    fps_counter.update();
    pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 0));
}

fn is_game_over(game_state: &GameState) -> bool {
    game_state.health <= 0
}

fn render_game_over_screen(game_state: &GameState, win_size: Vec2, pencil: &mut Pencil) {
    let msg = &format!("Game Over  -  score: {}", game_state.score);
    pencil.set_origin(win_size / 2 - Vec2::x(msg.len() / 2));
    pencil.draw_text(msg, Vec2::zero());
}

fn draw_game(game_state: &GameState, win_size: Vec2, mut pencil: Pencil) {
    pencil.set_origin((win_size - game_state.dimension) / 2);

    game_state.ship.draw(&mut pencil);
    for shot in &game_state.enemy_shots {
        shot.draw(&mut pencil);
    }
    for enemy in &game_state.enemies {
        enemy.draw(&mut pencil);
    }

    draw_hud(&game_state, &mut pencil);
    draw_border(game_state, &mut pencil);
}

fn draw_hud(game_state: &GameState, pencil: &mut Pencil) {
    pencil.set_foreground(Color::Red);
    let mut pos = Vec2::xy(-2, 0);
    pencil.draw_char('♥',pos);
    let curr_health_hud = game_state.health / HUD_HEALTH_GRANULARITY;
    for _ in 0..curr_health_hud {
        pos.y += 1;
        pencil.draw_char('|',pos);
    }
    pencil.set_foreground(Color::White);
    let status_msg = &format!("Health: {}  -  score: {}", game_state.health, game_state.score);
    pencil.draw_text(status_msg, Vec2::xy(20, -1));
}

fn draw_border(game_state: &GameState, pencil: &mut Pencil) {
    pencil.set_foreground(Color::Grey);
    let border_rect = game_state.dimension.add(Vec2::xy(BORDER_SIZE, BORDER_SIZE));
    pencil.draw_rect(&RectCharset::simple_round_lines(), Vec2::zero(), border_rect);
}
