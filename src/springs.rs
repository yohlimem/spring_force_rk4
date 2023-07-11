use nannou::prelude::*;
use crate::RK4::RK4;

#[derive(Debug)]
pub struct Spring {
    pub position:f32,
    pub velocity:f32, // position prime
    // pub force: f32,
    // pub mass: f32,
    // pub rest_length: f32,
    // pub stiffness: f32,
    // pub friction: f32,
}
// https://www.myphysicslab.com/explain/what-is-a-diff-eq-en.html

impl Spring {
    pub fn update(&mut self, time: f32, dt: f32){
        // in a 2d graph y = position, x = time
        self.velocity = RK4::solver(self.position, time, dt, Self::calculate_spring_force);
        // self.velocity = RK4::solver(self.velocity, time, dt, Self::calculate_dampen_force);
        self.position = RK4::solver(self.velocity, time, dt, Self::calculate_position);
    }
    // according to the position it will calculate the force applied to the spring mass
    pub fn calculate_spring_force(x: f32, time: f32) -> f32{
        // if the origin of the spring is in the origin we can just use x itself

        let force = -1.0 * x;

        force
    }
    // according to the velocity it will calculate the dampening force applied to the spring mass
    pub fn calculate_dampen_force(x: f32, _time: f32) -> f32{
        // if the origin of the spring is in the origin we can just use x itself
        let force = -0.5 * x;

        force
    }

    pub fn calculate_position(x: f32, _time: f32) -> f32 {
        x
    }
}

impl Spring {
    pub fn create(position: f32, velocity: f32) -> Self{
        Spring { position, velocity }
    }
}
