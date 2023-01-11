use std::cmp::{max, min};
use rand::{Rng, thread_rng};
use ruscii::spatial::{Vec2};
use crate::constant::{BORDER_SIZE, DMG_COLLISION, DMG_ENEMY_REACHED_GROUND, DMG_SHOT_HIT, FPS_LIMIT, INITIAL_HEALTH, INITIAL_SHIELD, INITIAL_SPAWN_INTERVAL, MAX_HEALTH, MAX_SHIELD, MIN_SPAWN_INTERVAL, SPAWN_INTERVAL_DECREASE, SPEEDUP_AFTER_X_FRAMES};
use crate::enemy::Enemy;
use crate::goodie::{Goodie, GoodieType};
use crate::ship::Ship;
use crate::shot::Shot;

pub struct GameState {
    pub dimension: Vec2,
    pub ship: Ship,
    pub enemies: Vec<Enemy>,
    pub enemy_shots: Vec<Shot>,
    pub goodies: Vec<Goodie>,
    pub health: isize,
    pub shield: usize,
    pub score: usize,
    pub last_spawn: usize,
    pub spawn_interval: usize,
    pub last_spawn_speedup: usize,
}

impl GameState {
    pub fn new(dimension: Vec2) -> GameState {
        let ship = Ship::new(Vec2::xy(dimension.x / 2, dimension.y - 2));
        GameState {
            dimension,
            ship,
            enemies: Vec::new(),
            enemy_shots: Vec::new(),
            goodies: Vec::new(),
            health: INITIAL_HEALTH,
            shield: INITIAL_SHIELD,
            score: 0,
            last_spawn: 0,
            spawn_interval: INITIAL_SPAWN_INTERVAL,
            last_spawn_speedup: 0,
        }
    }

    pub fn move_ship_x(&mut self, dx: i32) {
        self.ship.move_x(dx, self.dimension.x);
    }

    pub fn move_ship_y(&mut self, dy: i32) {
        self.ship.move_y(dy, self.dimension.y);
    }

    pub fn update(&mut self, frame: usize) {
        self.ship.update(frame);
        self.update_enemies(frame);
        self.update_enemy_shots();
        self.update_goodies();
        self.spawn_enemy(frame);
        self.update_ship_shots();
        self.update_game_speed(frame);
    }

    fn update_enemies(&mut self, frame: usize) {
        self.enemies.iter_mut().for_each(|enemy| enemy.update(frame, &mut self.enemy_shots));
        self.enemies.retain(|enemy| {
            if self.ship.is_hit_by(&enemy.pos) {
                self.health -= DMG_COLLISION;
                return false;
            }
            if enemy.pos.y > self.dimension.y - BORDER_SIZE {
                self.health -= DMG_ENEMY_REACHED_GROUND;
                return false;
            }
            true
        });
    }

    fn update_enemy_shots(&mut self) {
        self.enemy_shots.retain(|shot| {
            if self.ship.is_hit_by(&shot.pos) {
                self.health -= DMG_SHOT_HIT;
                return false;
            }
            shot.pos.y < self.dimension.y - BORDER_SIZE
        });
        self.enemy_shots.iter_mut().for_each(|shot| {
            shot.update();
        });
    }

    fn update_goodies(&mut self) {
        self.goodies.iter_mut().for_each(|goodie| goodie.update());
        self.goodies.retain(|goodie| {
            if self.ship.is_hit_by(&goodie.pos) {
                match &goodie.goodie_type {
                    GoodieType::RepairKit(additional_health) => {
                        self.health = min(self.health + *additional_health as isize, MAX_HEALTH as isize);
                    }
                    GoodieType::ShieldBoost(additional_shield) => {
                        self.shield = min(self.shield + additional_shield, MAX_SHIELD);
                    }
                    GoodieType::ShipUpgrade(new_ship_type) => {
                        self.ship.change_ship_type(new_ship_type);
                    }
                }
                return false;
            }
            goodie.pos.y < self.dimension.y - BORDER_SIZE
        });
    }

    fn spawn_enemy(&mut self, frame: usize) {
        if self.last_spawn + self.spawn_interval < frame {
            self.last_spawn = frame;
            let x_pos = thread_rng().gen_range(5..self.dimension.x - 5);
            self.enemies.push(
                Enemy::new(
                    Vec2::xy(x_pos, BORDER_SIZE),
                    Vec2::y(1),
                    FPS_LIMIT as usize,
                )
            );
        }
    }

    fn update_ship_shots(&mut self) {
        let mut partial_score = 0;
        let enemies = &mut self.enemies;
        self.ship.shots.retain(|shot| {
            if shot.y == 1 { return false; }
            let pre_len = enemies.len();
            enemies.retain(|enemy| {
                if &enemy.pos == shot {
                    partial_score += 5;
                    self.goodies.push(Goodie::new(enemy.pos, GoodieType::RepairKit(5)));
                    return false;
                }
                true
            });
            let destroyed = enemies.len() != pre_len;
            !destroyed
        });
        self.score += partial_score;
    }

    fn update_game_speed(&mut self, frame: usize) {
        if self.last_spawn_speedup + SPEEDUP_AFTER_X_FRAMES < frame {
            self.spawn_interval = max(MIN_SPAWN_INTERVAL, self.spawn_interval - SPAWN_INTERVAL_DECREASE);
            self.last_spawn_speedup = frame;
        }
    }
}
