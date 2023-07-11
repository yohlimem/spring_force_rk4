use nannou::prelude::*;

pub struct RK4{
    pub position: Vec2,
}

impl RK4 {

    pub fn solver(x: f32, current_time: f32, dt: f32, f: fn(f32, f32) -> f32) -> f32{
        let k1 = f(current_time ,x);
        let k2 = f(current_time + dt/2.0, x + (dt/2.0)*k1);
        let k3 = f(current_time + dt/2.0, x + (dt/2.0)*k2);
        let k4 = f(current_time + dt, x + (dt*k3));
        x + (dt/6.0)*(k1 + 2.0*k2 + 2.0*k3 + k4)
    }
}
