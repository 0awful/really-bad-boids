use std::f32::consts::PI;

use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

const X_MAX: real = 1000.0;
const X_MIN: real = 0.0;
const Y_MAX: real = 600.0;
const Y_MIN: real = 0.0;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub struct Boid {
    #[base]
    pub base: Base<CharacterBody2D>,
    #[export]
    #[init(default = 300.0)]
    pub max_speed: real,
    in_vision: Vec<Gd<Boid>>,
    in_flock: Vec<Gd<Boid>>,
}

// A boid has three properties:
// 1. Separation
// 2. Alignment
// 3. Cohesion
//
// Advanced rules are:
// 1. Obstacle Avoidance
// 2. Goal Seeking
// 3. Fleeing from enemies

#[godot_api]
impl Boid {
    #[func]
    pub fn separate(&self) -> Vector2 {
        let mut movement = Vector2::ZERO;
        self.in_vision.iter().for_each(|node| {
            movement = movement - (self.base.get_global_position() - node.get_global_position())
        });
        return movement;
    }

    #[func]
    pub fn cohesion(&self) -> Vector2 {
        let sums = self
            .in_flock
            .iter()
            .filter(|boid| *boid != &self.base.clone().cast::<Boid>())
            .fold(Vector2::ZERO, |acc, boid| {
                return acc + boid.clone().cast::<Boid>().get_global_position();
            });
        let movement =
            (sums / (self.in_flock.len() as real - 1.0)) - self.base.get_global_position();
        return movement / 100.0;
    }

    #[func]
    pub fn alignment(&self) -> Vector2 {
        return self.in_vision.iter().fold(Vector2::ZERO, |acc, node| {
            return acc + node.get_velocity();
        });
    }

    #[func]
    pub fn on_body_entered(&mut self, area: Gd<Node2D>) {
        if let Ok(area) = area.try_cast::<Boid>() {
            self.in_vision.push(area)
        }
    }
    #[func]
    pub fn on_body_exited(&mut self, area: Gd<Node2D>) {
        if let Ok(area) = area.try_cast::<Boid>() {
            self.in_vision = self
                .in_vision
                .iter()
                .filter(|boid| *boid != &area.clone())
                .map(|boid| boid.clone())
                .collect();
        }
    }

    #[func]
    pub fn flock_entered(&mut self, area: Gd<Node2D>) {
        if let Ok(area) = area.try_cast::<Boid>() {
            self.in_flock.push(area)
        }
    }
    #[func]
    pub fn flock_exited(&mut self, area: Gd<Node2D>) {
        if let Ok(area) = area.try_cast::<Boid>() {
            self.in_flock = self
                .in_flock
                .iter()
                .filter(|boid| *boid != &area.clone())
                .map(|boid| boid.clone())
                .collect();
        }
    }

    #[func]
    pub fn rotate(&mut self) {
        let angle = self.base.get_real_velocity().angle() + (PI / 2.0);
        self.base.set_rotation(angle);
    }

    #[func]
    pub fn bound(&mut self) -> Vector2 {
        let position = self.base.get_global_position();
        let mut movement = Vector2::ZERO;
        if position.x > X_MAX {
            movement.x -= 500.0;
        } else if position.x < X_MIN {
            movement.x += 500.0;
        }
        if position.y > Y_MAX {
            movement.y -= 500.0;
        } else if position.y < Y_MIN {
            movement.y += 500.0;
        }
        return movement;
    }
}

#[godot_api]
impl ICharacterBody2D for Boid {
    fn physics_process(&mut self, _: f64) {
        let cohesion = self.cohesion();
        let separate = self.separate();
        let alignment = self.alignment();
        let bound = self.bound();
        let mut movement_vec = cohesion + separate + alignment + bound;
        if movement_vec.length_squared() > self.max_speed * self.max_speed {
            movement_vec = movement_vec.normalized() * self.max_speed;
        }
        self.base.set_velocity(movement_vec);
        self.rotate();
        self.base.move_and_slide();
    }
}
