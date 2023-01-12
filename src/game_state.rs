use std::cmp::{max, min};
use rand::{Rng, thread_rng};
use ruscii::spatial::{Vec2};
use crate::constant::{BORDER_SIZE, CHAR_SHOT_SHIP_STRONG, DMG_COLLISION, DMG_ENEMY_REACHED_GROUND, DMG_SHOT_HIT, FPS_LIMIT, INITIAL_HEALTH, INITIAL_SHIELD, INITIAL_SPAWN_INTERVAL, MAX_HEALTH, MAX_SHIELD, MIN_SPAWN_INTERVAL, SPAWN_INTERVAL_DECREASE, SPEEDUP_AFTER_X_FRAMES};
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
    pub health: usize,
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
        self.ship.move_x(dx, self.dimension.x - 2);
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
        let mut standard_damage = 0;
        let mut direct_damage = 0;
        self.enemies.retain(|enemy| {
            if self.ship.is_hit_by(&enemy.pos) {
                standard_damage += DMG_COLLISION;
                return false;
            }
            if enemy.pos.y > self.dimension.y - BORDER_SIZE {
                direct_damage += DMG_ENEMY_REACHED_GROUND;
                return false;
            }
            true
        });
        self.damage_ship(standard_damage, direct_damage);
    }

    fn damage_ship(&mut self, damage: usize, direct_damage: usize) {
        self.health = if self.health <= direct_damage { 0 } else { self.health - direct_damage };
        if self.shield > 0 {
            self.shield = if self.shield <= damage { 0 } else { self.shield - damage };
        } else {
            self.health = if self.health <= damage { 0 } else { self.health - damage };
        }
    }

    fn update_enemy_shots(&mut self) {
        let mut standard_damage = 0;
        self.enemy_shots.iter_mut().for_each(|shot| { shot.update(); });
        self.enemy_shots.retain(|shot| {
            if self.ship.is_hit_by(&shot.pos) {
                standard_damage += DMG_SHOT_HIT;
                return false;
            }
            shot.pos.y < self.dimension.y - BORDER_SIZE
        });
        self.damage_ship(standard_damage, 0);
    }

    fn update_goodies(&mut self) {
        self.goodies.iter_mut().for_each(|goodie| goodie.update());
        self.goodies.retain(|goodie| {
            if self.ship.is_hit_by(&goodie.pos) {
                match &goodie.goodie_type {
                    GoodieType::RepairKit(additional_health) => {
                        self.health = min(self.health + *additional_health, MAX_HEALTH);
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
            if shot.pos.y == 0 { return false; }
            if shot.pos.x == 0 || shot.pos.x == self.dimension.x { return false; }
            let pre_len = enemies.len();
            enemies.retain(|enemy| {
                if enemy.pos == shot.pos {
                    partial_score += 5;
                    self.goodies.push(Goodie::new(enemy.pos, rand::random()));
                    return false;
                }
                true
            });
            let destroyed = enemies.len() != pre_len;
            !destroyed || shot.character == CHAR_SHOT_SHIP_STRONG
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
